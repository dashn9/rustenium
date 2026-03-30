use rustenium::browsers::{BidiBrowser, NavigateOptionsBuilder};
use rustenium::nodes::Node;
use rustenium::input::{Point, MouseClickOptions};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;
use rustenium_bidi_definitions::script::types::{RemoteValue, PrimitiveProtocolValue};
use rustenium_macros::css;

fn extract_string(value: &RemoteValue) -> Option<String> {
    match value {
        RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(sv)) => {
            Some(sv.value.clone())
        }
        _ => None,
    }
}

fn extract_number(value: &RemoteValue) -> Option<f64> {
    match value {
        RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::NumberValue(nv)) => {
            nv.value.as_f64()
        }
        _ => None,
    }
}

fn nav_opts() -> rustenium::browsers::NavigateOptions {
    NavigateOptionsBuilder::default()
        .wait(ReadinessState::Complete)
        .build()
}

pub async fn test_create_and_close(browser: impl BidiBrowser) {
    browser.close().await.unwrap();
}

pub async fn test_navigate_to_url(mut browser: impl BidiBrowser) {
    let result = browser.navigate("https://example.com").await;
    assert!(result.is_ok());
    browser.close().await.unwrap();
}

pub async fn test_navigate_with_wait_complete(mut browser: impl BidiBrowser) {
    let result = browser.navigate_with_options("https://example.com", nav_opts()).await;
    assert!(result.is_ok());
    browser.close().await.unwrap();
}

pub async fn test_navigate_multiple_pages(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    browser.navigate_with_options("https://httpbin.org/html", nav_opts()).await.unwrap();
    browser.close().await.unwrap();
}

pub async fn test_find_nodes_body(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find body element");
    browser.close().await.unwrap();
}

pub async fn test_find_nodes_heading(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find h1 on example.com");
    browser.close().await.unwrap();
}

pub async fn test_find_nodes_nonexistent_returns_empty(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("#nonexistent-element-xyz")).await.unwrap();
    assert!(nodes.is_empty(), "Should return empty vec for nonexistent selector");
    browser.close().await.unwrap();
}

pub async fn test_find_node_returns_first(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let node = browser.find_node(css!("h1")).await.unwrap();
    assert!(node.is_some(), "Should find first h1");
    browser.close().await.unwrap();
}

pub async fn test_find_node_nonexistent_returns_none(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let node = browser.find_node(css!("#nonexistent-element-xyz")).await.unwrap();
    assert!(node.is_none(), "Should return None for nonexistent selector");
    browser.close().await.unwrap();
}

pub async fn test_wait_for_nodes_existing(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.wait_for_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty(), "wait_for_nodes should find existing h1");
    browser.close().await.unwrap();
}

pub async fn test_wait_for_node_existing(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let node = browser.wait_for_node(css!("body")).await.unwrap();
    assert!(node.is_some(), "wait_for_node should find existing body");
    browser.close().await.unwrap();
}

pub async fn test_get_inner_text(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let text = nodes[0].get_inner_text().await;
    assert!(!text.is_empty(), "h1 inner text should not be empty");
    assert!(text.contains("Example Domain"), "h1 should contain 'Example Domain', got: {}", text);
    browser.close().await.unwrap();
}

pub async fn test_get_text_content(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let text = nodes[0].get_text_content().await;
    assert!(text.contains("Example Domain"), "text_content should contain 'Example Domain'");
    browser.close().await.unwrap();
}

pub async fn test_get_inner_html(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    let html = nodes[0].get_inner_html().await;
    assert!(html.contains("<h1>"), "body innerHTML should contain <h1>");
    browser.close().await.unwrap();
}

pub async fn test_get_attribute_href(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("a")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find anchor elements");
    let href = nodes[0].get_attribute("href");
    assert!(href.is_some(), "Anchor should have href attribute");
    browser.close().await.unwrap();
}

pub async fn test_get_children_nodes(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    let children = nodes[0].get_children_nodes();
    assert!(!children.is_empty(), "body should have child nodes");
    browser.close().await.unwrap();
}

pub async fn test_node_context_id(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context_id = browser.get_active_context_id().unwrap();
    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert_eq!(nodes[0].get_context_id(), &context_id);
    browser.close().await.unwrap();
}

pub async fn test_node_is_visible(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let visible = nodes[0].is_visible().await.unwrap();
    assert!(visible, "h1 should be visible");
    browser.close().await.unwrap();
}

pub async fn test_node_scroll_into_view(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("p")).await.unwrap();
    let result = nodes[0].scroll_into_view().await;
    assert!(result.is_ok(), "scroll_into_view should succeed");
    browser.close().await.unwrap();
}

pub async fn test_node_get_position(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let mut nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let pos = nodes[0].get_position().await;
    assert!(pos.is_some(), "h1 should have a position");
    let pos = pos.unwrap();
    assert!(pos.width > 0.0, "h1 width should be positive");
    assert!(pos.height > 0.0, "h1 height should be positive");
    browser.close().await.unwrap();
}

pub async fn test_node_mouse_move(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let mut nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let result = nodes[0].mouse_move().await;
    assert!(result.is_ok(), "mouse_move to h1 should succeed");
    browser.close().await.unwrap();
}

pub async fn test_node_mouse_click(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let mut nodes = browser.find_nodes(css!("a")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find a link to click");
    let result = nodes[0].mouse_click().await;
    assert!(result.is_ok(), "mouse_click on link should succeed");
    browser.close().await.unwrap();
}

pub async fn test_mouse_move_to_point(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    let result = mouse.move_to(Point { x: 100.0, y: 200.0 }, &context, Default::default()).await;
    assert!(result.is_ok(), "mouse move_to should succeed");
    browser.close().await.unwrap();
}

pub async fn test_mouse_click_at_point(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    let result = mouse.click(Some(Point { x: 100.0, y: 100.0 }), &context, MouseClickOptions::default()).await;
    assert!(result.is_ok(), "mouse click should succeed");
    browser.close().await.unwrap();
}

pub async fn test_mouse_down_and_up(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    mouse.down(&context, Default::default()).await.unwrap();
    mouse.up(&context, Default::default()).await.unwrap();
    browser.close().await.unwrap();
}

pub async fn test_keyboard_type_text(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    let result = keyboard.type_text("hello", &context, None).await;
    assert!(result.is_ok(), "keyboard type_text should succeed");
    browser.close().await.unwrap();
}

pub async fn test_keyboard_press_key(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    let result = keyboard.press("Enter", &context, None).await;
    assert!(result.is_ok(), "keyboard press Enter should succeed");
    browser.close().await.unwrap();
}

pub async fn test_keyboard_down_and_up(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    keyboard.down("Shift", &context).await.unwrap();
    keyboard.up("Shift", &context).await.unwrap();
    browser.close().await.unwrap();
}

pub async fn test_evaluate_script_simple(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let result = browser.evaluate_script("1 + 1".to_string(), false).await;
    assert!(result.is_ok(), "evaluate_script should succeed");
    browser.close().await.unwrap();
}

pub async fn test_evaluate_script_returns_string(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let result = browser.evaluate_script("document.title".to_string(), false).await.unwrap();
    let title = extract_string(&result.result).expect("Should return a string");
    assert!(title.contains("Example Domain"), "title should contain 'Example Domain', got: {}", title);
    browser.close().await.unwrap();
}

pub async fn test_evaluate_script_returns_number(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let result = browser.evaluate_script("document.querySelectorAll('p').length".to_string(), false).await.unwrap();
    let count = extract_number(&result.result).expect("Should return a number");
    assert!(count > 0.0, "Should find at least one paragraph");
    browser.close().await.unwrap();
}

pub async fn test_create_new_context(mut browser: impl BidiBrowser) {
    let new_context = browser.create_context(false).await;
    assert!(new_context.is_ok(), "create_context should succeed");
    browser.close().await.unwrap();
}

pub async fn test_get_active_context_id(browser: impl BidiBrowser) {
    let context = browser.get_active_context_id();
    assert!(context.is_ok(), "Should have an active context after creation");
    browser.close().await.unwrap();
}

pub async fn test_screenshot_returns_base64(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let data = browser.screenshot().await.unwrap();
    assert!(!data.is_empty(), "Screenshot data should not be empty");
    browser.close().await.unwrap();
}

pub async fn test_node_screenshot_returns_base64(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let data = nodes[0].screenshot().await.unwrap();
    assert!(!data.is_empty(), "Node screenshot data should not be empty");
    browser.close().await.unwrap();
}

pub async fn test_emulate_timezone(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    browser.emulate_timezone(Some("America/New_York".to_string())).await.unwrap();
    let eval = browser.evaluate_script(
        "Intl.DateTimeFormat().resolvedOptions().timeZone".to_string(),
        false,
    ).await.unwrap();
    let tz = extract_string(&eval.result).expect("Should return timezone string");
    assert_eq!(tz, "America/New_York", "Timezone should be emulated");
    browser.close().await.unwrap();
}

pub async fn test_delete_node(mut browser: impl BidiBrowser) {
    browser.navigate_with_options("https://example.com", nav_opts()).await.unwrap();
    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty());
    nodes[0].delete().await.unwrap();
    let nodes_after = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(nodes_after.is_empty(), "h1 should be deleted");
    browser.close().await.unwrap();
}

pub async fn test_end_session_cleanly(mut browser: impl BidiBrowser) {
    let result = browser.end_session().await;
    assert!(result.is_ok(), "end_session should succeed");
}
