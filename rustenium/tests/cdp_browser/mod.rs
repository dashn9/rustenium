use rustenium::browsers::{
    BidiBrowser, ChromeBrowser,
    cdp_browser::{AddPreloadScriptOptions, CdpBrowser, FetchNodeOptions, WaitUntil},
};
use rustenium::nodes::{AXNode, Node};
use rustenium_cdp_definitions::browser_protocol::dom::types::BackendNodeId;
use rustenium_cdp_definitions::js_protocol::runtime::commands::Enable as RuntimeEnable;
use rustenium_core::{HttpRequest, ResponseForRequest};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

pub async fn test_wait_for_navigation(mut browser: ChromeBrowser) {
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://example.com")
        .await
        .unwrap();

    <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "setTimeout(() => { location.href = 'https://example.com/?nav=1'; }, 500)",
        false,
    )
    .await
    .unwrap();

    <ChromeBrowser as CdpBrowser>::wait_for_navigation(
        &mut browser,
        WaitUntil::Load,
        Duration::from_secs(30),
    )
    .await
    .expect("should observe a cross-document navigation started after arming");

    let loaded = <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "location.search + '|' + document.readyState",
        false,
    )
    .await
    .unwrap();
    assert_eq!(
        loaded.result.value,
        Some(serde_json::json!("?nav=1|complete")),
        "should return only once the new document has finished loading"
    );

    <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "setTimeout(() => { history.pushState({}, '', '/?nav=2'); }, 500)",
        false,
    )
    .await
    .unwrap();

    <ChromeBrowser as CdpBrowser>::wait_for_navigation(
        &mut browser,
        WaitUntil::Load,
        Duration::from_secs(30),
    )
    .await
    .expect("should observe a same-document navigation, which fires no lifecycle events");

    let pushed =
        <ChromeBrowser as CdpBrowser>::evaluate_script(&mut browser, "location.search", false)
            .await
            .unwrap();
    assert_eq!(pushed.result.value, Some(serde_json::json!("?nav=2")));

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

pub async fn test_request_interception(mut browser: ChromeBrowser) {
    let seen: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let cooperated = Arc::new(AtomicBool::new(false));

    let sink = seen.clone();
    <ChromeBrowser as CdpBrowser>::on_request(&mut browser, move |request| {
        let sink = sink.clone();
        async move {
            sink.lock().unwrap().push((
                request.url().to_string(),
                HttpRequest::method(request.as_ref()).to_string(),
            ));

            if request.url().ends_with("/blocked") {
                let _ = request.abort().await;
            } else if request.url().ends_with("/mocked") {
                let _ = request
                    .respond(
                        ResponseForRequest::new(200)
                            .content_type("text/plain")
                            .body("intercepted"),
                    )
                    .await;
            } else {
                let _ = request
                    .respond(
                        ResponseForRequest::new(200)
                            .content_type("text/html")
                            .body("<html><body>hermetic</body></html>"),
                    )
                    .await;
            }
        }
    })
    .await
    .expect("on_request should enable Fetch");

    // A second handler sees the same request object, after the first resolved it.
    let flag = cooperated.clone();
    <ChromeBrowser as CdpBrowser>::on_request(&mut browser, move |request| {
        let flag = flag.clone();
        async move {
            if request.is_handled() {
                flag.store(true, Ordering::SeqCst);
            }
        }
    })
    .await
    .expect("a second on_request should merge into the existing interception");

    // Fulfilled from the handler above, so the test never touches the network.
    <ChromeBrowser as CdpBrowser>::navigate(&mut browser, "https://rustenium.test/")
        .await
        .unwrap();

    let mocked = <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "fetch('/mocked').then(r => r.text())",
        true,
    )
    .await
    .unwrap();
    assert_eq!(
        mocked.result.value,
        Some(serde_json::json!("intercepted")),
        "respond should deliver the mocked body to the page"
    );

    let blocked = <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "fetch('/blocked').then(() => 'loaded', () => 'blocked')",
        true,
    )
    .await
    .unwrap();
    assert_eq!(
        blocked.result.value,
        Some(serde_json::json!("blocked")),
        "abort should make the request fail in the page"
    );

    let requests = seen.lock().unwrap().clone();
    assert!(
        requests
            .iter()
            .any(|(url, method)| url.ends_with("/mocked") && method == "GET"),
        "handler should have seen the mocked GET, saw {:?}",
        requests
    );
    assert!(
        requests.iter().any(|(url, _)| url.ends_with("/blocked")),
        "handler should have seen the blocked request, saw {:?}",
        requests
    );
    assert!(
        cooperated.load(Ordering::SeqCst),
        "second handler should observe the first one having resolved the request"
    );

    <ChromeBrowser as CdpBrowser>::disable_request_interception(&mut browser)
        .await
        .expect("disable_request_interception should succeed");

    browser.close().await.unwrap();
}

pub async fn test_events_handler_and_wait(mut browser: ChromeBrowser) {
    let logged: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // A CDP domain emits nothing until it is enabled.
    <ChromeBrowser as CdpBrowser>::send_command(&mut browser, RuntimeEnable::builder().build())
        .await
        .expect("Runtime.enable should succeed");

    let sink = logged.clone();
    let handler_id = <ChromeBrowser as CdpBrowser>::add_event_handler(
        &mut browser,
        ["Runtime.consoleAPICalled"],
        move |event| {
            let sink = sink.clone();
            async move {
                if let Some(text) = event.params["args"][0]["value"].as_str() {
                    sink.lock().unwrap().push(text.to_string());
                }
            }
        },
    )
    .await;
    assert!(!handler_id.is_empty(), "handler id should not be empty");

    <ChromeBrowser as CdpBrowser>::evaluate_script(&mut browser, "console.log('first')", false)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        logged.lock().unwrap().iter().any(|text| text == "first"),
        "handler should receive events from the enabled domain, saw {:?}",
        logged.lock().unwrap()
    );

    // Fires after wait_for_event has registered, which is the order it needs.
    <ChromeBrowser as CdpBrowser>::evaluate_script(
        &mut browser,
        "setTimeout(() => console.log('later'), 300)",
        false,
    )
    .await
    .unwrap();
    let event = <ChromeBrowser as CdpBrowser>::wait_for_event(
        &mut browser,
        ["Runtime.consoleAPICalled"],
        |event| event.params["args"][0]["value"] == serde_json::json!("later"),
        Duration::from_secs(5),
    )
    .await
    .expect("wait_for_event should resolve on the matching event");
    assert_eq!(event.method, "Runtime.consoleAPICalled");

    <ChromeBrowser as CdpBrowser>::remove_event_handler(&mut browser, &handler_id).await;
    logged.lock().unwrap().clear();
    <ChromeBrowser as CdpBrowser>::evaluate_script(&mut browser, "console.log('after')", false)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        logged.lock().unwrap().is_empty(),
        "removed handler should stop receiving events, saw {:?}",
        logged.lock().unwrap()
    );

    browser.close().await.unwrap();
}
