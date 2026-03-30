use rustenium::browsers::{chrome, BidiBrowser, ChromeBrowser, ChromeConfig, NavigateOptionsBuilder};
use rustenium::nodes::Node;
use rustenium::input::{Point, MouseClickOptions};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;
use rustenium_bidi_definitions::script::types::{RemoteValue, PrimitiveProtocolValue};
use rustenium_macros::css;



// Auto attach mode is used because in manual mode chromedriver spawns Chrome
// as its own child process, and Chrome's stdout/stderr (DevTools listening, GPU
// errors) leak directly to the terminal with no reliable way to suppress them.
// In auto attach mode Rustenium spawns Chrome itself via Process::create which
// pipes stdout/stderr, keeping test output clean.
async fn launch_headless() -> ChromeBrowser {
    let mut config = ChromeConfig::default();
    config.remote_debugging_port = Some(0);
    config.browser_flags = Some(vec![
        "--headless=new".to_string(),
        "--window-size=1280,720".to_string(),
    ]);
    chrome(Some(config)).await
}

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
// ── Browser creation ─────────────────────────────────────────────────────────

#[tokio::test]
async fn create_browser_default_config() {
    let browser = launch_headless().await;
    browser.close().await.unwrap();
}

#[tokio::test]
async fn create_browser_auto_attach_mode() {
    let mut config = ChromeConfig::default();
    config.remote_debugging_port = Some(0);
    config.browser_flags = Some(vec![
        "--headless=new".to_string(),
        "--window-size=1280,720".to_string(),
        "--disable-gpu".to_string(),
        "--log-level=3".to_string(),
        "--silent".to_string(),
    ]);

    let browser = chrome(Some(config)).await;
    browser.close().await.unwrap();
}

#[tokio::test]
async fn create_browser_with_custom_capabilities() {
    let mut config = ChromeConfig::default();
    config.remote_debugging_port = Some(0);
    config.browser_flags = Some(vec![
        "--headless=new".to_string(),
        "--window-size=1280,720".to_string(),
    ]);
    config.capabilities.accept_insecure_certs(true);

    let browser = chrome(Some(config)).await;
    browser.close().await.unwrap();
}

#[tokio::test]
async fn create_browser_manual_mode() {
    let mut config = ChromeConfig::default();
    config.capabilities
        .add_arg("--headless=new")
        .add_arg("--window-size=1280,720");
    let browser = chrome(Some(config)).await;
    browser.close().await.unwrap();
}

// ── Navigation ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn navigate_to_url() {
    let mut browser = launch_headless().await;
    let result = browser.navigate("https://example.com").await;
    assert!(result.is_ok());
    browser.close().await.unwrap();
}

#[tokio::test]
async fn navigate_with_wait_complete() {
    let mut browser = launch_headless().await;
    let result = browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await;
    assert!(result.is_ok());
    browser.close().await.unwrap();
}

#[tokio::test]
async fn navigate_multiple_pages() {
    let mut browser = launch_headless().await;

    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    browser.navigate_with_options("https://httpbin.org/html", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    browser.close().await.unwrap();
}

// ── Element finding ──────────────────────────────────────────────────────────

#[tokio::test]
async fn find_nodes_body() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find body element");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_nodes_heading() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find h1 on example.com");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_nodes_paragraph() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("p")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find paragraphs on example.com");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_nodes_link() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("a")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find anchor elements on example.com");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_nodes_nonexistent_returns_empty() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("#nonexistent-element-xyz")).await.unwrap();
    assert!(nodes.is_empty(), "Should return empty vec for nonexistent selector");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_node_returns_first() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let node = browser.find_node(css!("h1")).await.unwrap();
    assert!(node.is_some(), "Should find first h1");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn find_node_nonexistent_returns_none() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let node = browser.find_node(css!("#nonexistent-element-xyz")).await.unwrap();
    assert!(node.is_none(), "Should return None for nonexistent selector");
    browser.close().await.unwrap();
}

// ── Wait for nodes ───────────────────────────────────────────────────────────

#[tokio::test]
async fn wait_for_nodes_existing() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.wait_for_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty(), "wait_for_nodes should find existing h1");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn wait_for_node_existing() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let node = browser.wait_for_node(css!("body")).await.unwrap();
    assert!(node.is_some(), "wait_for_node should find existing body");
    browser.close().await.unwrap();
}

// ── Node properties ──────────────────────────────────────────────────────────

#[tokio::test]
async fn get_inner_text() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let text = nodes[0].get_inner_text().await;
    assert!(!text.is_empty(), "h1 inner text should not be empty");
    assert!(text.contains("Example Domain"), "h1 should contain 'Example Domain', got: {}", text);
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_text_content() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let text = nodes[0].get_text_content().await;
    assert!(text.contains("Example Domain"), "text_content should contain 'Example Domain'");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_inner_html() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    let html = nodes[0].get_inner_html().await;
    assert!(html.contains("<h1>"), "body innerHTML should contain <h1>");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_attribute_href() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("a")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find anchor elements");
    let href = nodes[0].get_attribute("href");
    assert!(href.is_some(), "Anchor should have href attribute");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_attributes_returns_map() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("a")).await.unwrap();
    let attrs = nodes[0].get_attributes();
    assert!(attrs.contains_key("href"), "Anchor attributes should include href");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_children_nodes() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    let children = nodes[0].get_children_nodes();
    assert!(!children.is_empty(), "body should have child nodes");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn node_context_id() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context_id = browser.get_active_context_id().unwrap();
    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert_eq!(nodes[0].get_context_id(), &context_id);
    browser.close().await.unwrap();
}

// ── Node visibility, scroll, position ────────────────────────────────────────

#[tokio::test]
async fn node_is_visible() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let visible = nodes[0].is_visible().await.unwrap();
    assert!(visible, "h1 should be visible");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn node_scroll_into_view() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("p")).await.unwrap();
    let result = nodes[0].scroll_into_view().await;
    assert!(result.is_ok(), "scroll_into_view should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn node_get_position() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let mut nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let pos = nodes[0].get_position().await;
    assert!(pos.is_some(), "h1 should have a position");
    let pos = pos.unwrap();
    assert!(pos.width > 0.0, "h1 width should be positive");
    assert!(pos.height > 0.0, "h1 height should be positive");
    browser.close().await.unwrap();
}

// ── Mouse interactions on nodes ──────────────────────────────────────────────

#[tokio::test]
async fn node_mouse_move() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let mut nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let result = nodes[0].mouse_move().await;
    assert!(result.is_ok(), "mouse_move to h1 should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn node_mouse_click() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let mut nodes = browser.find_nodes(css!("a")).await.unwrap();
    assert!(!nodes.is_empty(), "Should find a link to click");
    let result = nodes[0].mouse_click().await;
    assert!(result.is_ok(), "mouse_click on link should succeed");
    browser.close().await.unwrap();
}

// ── Direct mouse API ─────────────────────────────────────────────────────────

#[tokio::test]
async fn mouse_move_to_point() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    let result = mouse.move_to(Point { x: 100.0, y: 200.0 }, &context, Default::default()).await;
    assert!(result.is_ok(), "mouse move_to should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn mouse_click_at_point() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    let result = mouse.click(Some(Point { x: 100.0, y: 100.0 }), &context, MouseClickOptions::default()).await;
    assert!(result.is_ok(), "mouse click should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn mouse_down_and_up() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let mouse = browser.mouse();
    mouse.down(&context, Default::default()).await.unwrap();
    mouse.up(&context, Default::default()).await.unwrap();
    browser.close().await.unwrap();
}

// ── Keyboard ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn keyboard_type_text() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    let result = keyboard.type_text("hello", &context, None).await;
    assert!(result.is_ok(), "keyboard type_text should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn keyboard_press_key() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    let result = keyboard.press("Enter", &context, None).await;
    assert!(result.is_ok(), "keyboard press Enter should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn keyboard_down_and_up() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let context = browser.get_active_context_id().unwrap();
    let keyboard = browser.keyboard();
    keyboard.down("Shift", &context).await.unwrap();
    keyboard.up("Shift", &context).await.unwrap();
    browser.close().await.unwrap();
}

// ── Script evaluation ────────────────────────────────────────────────────────

#[tokio::test]
async fn evaluate_script_simple() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let result = browser.evaluate_script("1 + 1".to_string(), false).await;
    assert!(result.is_ok(), "evaluate_script should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn evaluate_script_returns_string() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let result = browser.evaluate_script("document.title".to_string(), false).await.unwrap();
    let title = extract_string(&result.result).expect("Should return a string");
    assert!(title.contains("Example Domain"), "title should contain 'Example Domain', got: {}", title);
    browser.close().await.unwrap();
}

#[tokio::test]
async fn evaluate_script_returns_number() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let result = browser.evaluate_script(
        "document.querySelectorAll('p').length".to_string(),
        false,
    ).await.unwrap();
    let count = extract_number(&result.result).expect("Should return a number");
    assert!(count > 0.0, "Should find at least one paragraph");
    browser.close().await.unwrap();
}

// ── Context management ───────────────────────────────────────────────────────

#[tokio::test]
async fn create_new_context() {
    let mut browser = launch_headless().await;
    let new_context = browser.create_context(false).await;
    assert!(new_context.is_ok(), "create_context should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn create_background_context() {
    let mut browser = launch_headless().await;
    let new_context = browser.create_context(true).await;
    assert!(new_context.is_ok(), "create background context should succeed");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn get_active_context_id() {
    let browser = launch_headless().await;
    let context = browser.get_active_context_id();
    assert!(context.is_ok(), "Should have an active context after creation");
    browser.close().await.unwrap();
}

// ── Screenshots ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn screenshot_returns_base64() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let data = browser.screenshot().await.unwrap();
    assert!(!data.is_empty(), "Screenshot data should not be empty");
    browser.close().await.unwrap();
}

#[tokio::test]
async fn node_screenshot_returns_base64() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    let data = nodes[0].screenshot().await.unwrap();
    assert!(!data.is_empty(), "Node screenshot data should not be empty");
    browser.close().await.unwrap();
}

// ── Timezone emulation ───────────────────────────────────────────────────────

#[tokio::test]
async fn emulate_timezone() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    browser.emulate_timezone(Some("America/New_York".to_string())).await.unwrap();

    let eval = browser.evaluate_script(
        "Intl.DateTimeFormat().resolvedOptions().timeZone".to_string(),
        false,
    ).await.unwrap();
    let tz = extract_string(&eval.result).expect("Should return timezone string");
    assert_eq!(tz, "America/New_York", "Timezone should be emulated");

    browser.close().await.unwrap();
}

// ── Node deletion ────────────────────────────────────────────────────────────

#[tokio::test]
async fn delete_node() {
    let mut browser = launch_headless().await;
    browser.navigate_with_options("https://example.com", NavigateOptionsBuilder::default().wait(ReadinessState::Complete).build()).await.unwrap();

    let nodes = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(!nodes.is_empty());
    nodes[0].delete().await.unwrap();

    let nodes_after = browser.find_nodes(css!("h1")).await.unwrap();
    assert!(nodes_after.is_empty(), "h1 should be deleted");
    browser.close().await.unwrap();
}

// ── Session end ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn end_session_cleanly() {
    let mut browser = launch_headless().await;
    let result = browser.end_session().await;
    assert!(result.is_ok(), "end_session should succeed");
}

#[tokio::test]
async fn close_browser() {
    let browser = launch_headless().await;
    browser.close().await.unwrap();
}
