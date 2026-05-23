use rustenium::browsers::{
    BidiBrowser, ChromeBrowser,
    cdp_browser::{AddPreloadScriptOptions, CdpBrowser, FetchNodeOptions},
};
use rustenium::nodes::{AXNode, Node};
use rustenium_cdp_definitions::browser_protocol::dom::types::BackendNodeId;

async fn first_backend_node_id(browser: &mut ChromeBrowser, url: &str) -> BackendNodeId {
    <ChromeBrowser as CdpBrowser>::navigate(browser, url)
        .await
        .unwrap();
    let nodes = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(browser, false)
        .await
        .unwrap();
    let id = nodes
        .iter()
        .find_map(|n| n.backend_dom_node_id)
        .expect("expected at least one accessibility node with a backend_dom_node_id");
    BackendNodeId::new(id)
}

pub async fn test_navigate_to_url(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await;
    assert!(result.is_ok(), "navigate should succeed");
    browser.close().await.unwrap();
}

pub async fn test_navigate_returns_frame_or_loader(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    assert!(
        !result.frame_id.inner().is_empty() || result.loader_id.is_some(),
        "navigate result should contain frame or loader id"
    );
    assert!(
        result.error_text.is_none(),
        "navigation should not error, got: {:?}",
        result.error_text
    );
    browser.close().await.unwrap();
}

pub async fn test_navigate_multiple_pages(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://httpbin.org/html")
        .await
        .unwrap();
    browser.close().await.unwrap();
}

pub async fn test_get_accessible_nodes_not_empty(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    let nodes = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, false)
        .await
        .unwrap();
    assert!(
        !nodes.is_empty(),
        "should have at least one accessibility node"
    );
    browser.close().await.unwrap();
}

pub async fn test_get_accessible_nodes_squash_lte_full(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    let full = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, false)
        .await
        .unwrap();
    let squashed = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, true)
        .await
        .unwrap();

    fn count(nodes: &[AXNode]) -> usize {
        nodes.iter().map(|n| 1 + count(&n.children)).sum()
    }
    assert!(
        count(&squashed) <= count(&full),
        "squashed tree ({}) should have <= nodes than full tree ({})",
        count(&squashed),
        count(&full)
    );
    browser.close().await.unwrap();
}

pub async fn test_get_accessible_nodes_squash_no_blank_generic(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    let nodes = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, true)
        .await
        .unwrap();

    fn is_blank_generic(node: &AXNode) -> bool {
        let name_blank = node.name_str().is_none_or(|s| s.trim().is_empty());
        let role_empty = node
            .role_str()
            .is_none_or(|r| matches!(r, "none" | "generic" | "group"));
        name_blank && role_empty
    }

    fn check(nodes: &[AXNode], parent_blank_generic: bool) {
        for node in nodes {
            let blank = is_blank_generic(node);
            assert!(
                !(blank && parent_blank_generic),
                "squashed tree should not contain a blank-generic child of a blank-generic parent, found node_id: {}",
                node.node_id
            );
            check(&node.children, blank);
        }
    }
    check(&nodes, false);
    browser.close().await.unwrap();
}

pub async fn test_fetch_node_by_node_id(mut browser: ChromeBrowser) {
    let backend_id = first_backend_node_id(&mut browser, "https://example.com").await;
    let node = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().backend_node_id(backend_id).depth(1),
    )
    .await
    .unwrap();
    assert!(
        node.get_node_type().is_some(),
        "fetched node should have a valid node type"
    );
    browser.close().await.unwrap();
}

pub async fn test_fetch_node_with_depth_returns_children(mut browser: ChromeBrowser) {
    let backend_id = first_backend_node_id(&mut browser, "https://example.com").await;
    let shallow = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().backend_node_id(backend_id).depth(1),
    )
    .await
    .unwrap();
    let deep = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().backend_node_id(backend_id).depth(5),
    )
    .await
    .unwrap();
    assert!(
        deep.get_children_nodes().len() >= shallow.get_children_nodes().len(),
        "deeper fetch should return at least as many children"
    );
    browser.close().await.unwrap();
}

pub async fn test_emulate_device_metrics(mut browser: ChromeBrowser) {
    let result =
        <ChromeBrowser as CdpBrowser>::emulate_device_metrics(&mut browser, 375, 812, 3.0, true)
            .await;
    assert!(result.is_ok(), "emulate_device_metrics should succeed");
    browser.close().await.unwrap();
}

pub async fn test_create_tab(mut browser: ChromeBrowser) {
    let result =
        <ChromeBrowser as CdpBrowser>::create_tab(&mut browser, "https://example.com").await;
    assert!(result.is_ok(), "create_tab should succeed");
    browser.close().await.unwrap();
}

pub async fn test_preload_script_add_runs_and_remove(mut browser: ChromeBrowser) {
    let script_id = <ChromeBrowser as CdpBrowser>::add_preload_script_with_options(
        &mut browser,
        "globalThis.__rustenium_preload__ = 'ran';",
        AddPreloadScriptOptions::default().run_immediately(true),
    )
    .await
    .expect("add_preload_script_with_options should succeed");
    assert!(
        !script_id.inner().is_empty(),
        "identifier should not be empty"
    );

    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();
    let ran = <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "globalThis.__rustenium_preload__",
        false,
    )
    .await
    .unwrap();
    assert_eq!(
        ran.result.value,
        Some(serde_json::json!("ran")),
        "preload script should set the global on navigation"
    );

    <ChromeBrowser as CdpBrowser>::remove_preload_script(&mut browser, script_id)
        .await
        .expect("remove_preload_script should succeed");
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.org")
        .await
        .unwrap();
    let after = <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "typeof globalThis.__rustenium_preload__",
        false,
    )
    .await
    .unwrap();
    assert_eq!(
        after.result.value,
        Some(serde_json::json!("undefined")),
        "removed preload should not run on subsequent navigation"
    );

    browser.close().await.unwrap();
}
