use rustenium::browsers::{ChromeBrowser, BidiBrowser, cdp_browser::{CdpBrowser, FetchNodeOptions}};
use rustenium::nodes::{AXNode, Node};
use rustenium_cdp_definitions::browser_protocol::dom::types::NodeId;

pub async fn test_navigate_to_url(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await;
    assert!(result.is_ok(), "navigate should succeed");
    browser.close().await.unwrap();
}

pub async fn test_navigate_returns_frame_or_loader(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    assert!(
        !result.frame_id.inner().is_empty() || result.loader_id.is_some(),
        "navigate result should contain frame or loader id"
    );
    assert!(result.error_text.is_none(), "navigation should not error, got: {:?}", result.error_text);
    browser.close().await.unwrap();
}

pub async fn test_navigate_multiple_pages(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://httpbin.org/html").await.unwrap();
    browser.close().await.unwrap();
}

pub async fn test_get_accessible_nodes_not_empty(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    let nodes = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, false).await.unwrap();
    assert!(!nodes.is_empty(), "should have at least one accessibility node");
    browser.close().await.unwrap();
}

pub async fn test_get_accessible_nodes_squash_lte_full(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    let full = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, false).await.unwrap();
    let squashed = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, true).await.unwrap();

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
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    let nodes = <ChromeBrowser as CdpBrowser>::get_accessible_nodes(&mut browser, true).await.unwrap();

    fn check(nodes: &[AXNode]) {
        for node in nodes {
            let name_blank = node.name_str().map_or(true, |s| s.trim().is_empty());
            let role_empty = node.role_str().map_or(true, |r| matches!(r, "none" | "generic" | "group"));
            assert!(
                !(name_blank && role_empty),
                "squashed tree should contain no blank generic nodes, found node_id: {}",
                node.node_id
            );
            check(&node.children);
        }
    }
    check(&nodes);
    browser.close().await.unwrap();
}

pub async fn test_fetch_node_by_node_id(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    let node = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().node_id(NodeId::new(1)).depth(1),
    ).await.unwrap();
    assert!(node.get_node_type().is_some(), "fetched node should have a valid node type");
    browser.close().await.unwrap();
}

pub async fn test_fetch_node_with_depth_returns_children(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com").await.unwrap();
    let shallow = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().node_id(NodeId::new(1)).depth(1),
    ).await.unwrap();
    let deep = <ChromeBrowser as CdpBrowser>::fetch_node(
        &mut browser,
        FetchNodeOptions::new().node_id(NodeId::new(1)).depth(5),
    ).await.unwrap();
    assert!(
        deep.get_children_nodes().len() >= shallow.get_children_nodes().len(),
        "deeper fetch should return at least as many children"
    );
    browser.close().await.unwrap();
}

pub async fn test_emulate_device_metrics(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::emulate_device_metrics(&mut browser, 375, 812, 3.0, true).await;
    assert!(result.is_ok(), "emulate_device_metrics should succeed");
    browser.close().await.unwrap();
}

pub async fn test_create_tab(mut browser: ChromeBrowser) {
    let result = <ChromeBrowser as CdpBrowser>::create_tab(&mut browser, "https://example.com").await;
    assert!(result.is_ok(), "create_tab should succeed");
    browser.close().await.unwrap();
}
