mod bidi_browser;
mod cdp_browser;

use rustenium::browsers::{ChromeBrowser, ChromeConfig, chrome};

async fn launch() -> ChromeBrowser {
    let config = ChromeConfig {
        remote_debugging_port: Some(0),
        browser_flags: Some(vec![
            "--headless=new".to_string(),
            "--window-size=1280,720".to_string(),
        ]),
        ..ChromeConfig::default()
    };
    chrome(Some(config)).await
}

async fn launch_cdp() -> ChromeBrowser {
    let config = ChromeConfig {
        enable_bidi: false,
        enable_cdp: true,
        browser_flags: Some(vec![
            "--headless=new".to_string(),
            "--window-size=1280,720".to_string(),
        ]),
        ..ChromeConfig::default()
    };
    chrome(Some(config)).await
}

#[tokio::test]
async fn create_and_close() {
    bidi_browser::test_create_and_close(launch().await).await;
}
#[tokio::test]
async fn navigate_to_url() {
    bidi_browser::test_navigate_to_url(launch().await).await;
}
#[tokio::test]
async fn navigate_with_wait_complete() {
    bidi_browser::test_navigate_with_wait_complete(launch().await).await;
}
#[tokio::test]
async fn navigate_multiple_pages() {
    bidi_browser::test_navigate_multiple_pages(launch().await).await;
}
#[tokio::test]
async fn find_nodes_body() {
    bidi_browser::test_find_nodes_body(launch().await).await;
}
#[tokio::test]
async fn find_nodes_heading() {
    bidi_browser::test_find_nodes_heading(launch().await).await;
}
#[tokio::test]
async fn find_nodes_nonexistent_returns_empty() {
    bidi_browser::test_find_nodes_nonexistent_returns_empty(launch().await).await;
}
#[tokio::test]
async fn find_node_returns_first() {
    bidi_browser::test_find_node_returns_first(launch().await).await;
}
#[tokio::test]
async fn find_node_nonexistent_returns_none() {
    bidi_browser::test_find_node_nonexistent_returns_none(launch().await).await;
}
#[tokio::test]
async fn wait_for_nodes_existing() {
    bidi_browser::test_wait_for_nodes_existing(launch().await).await;
}
#[tokio::test]
async fn wait_for_node_existing() {
    bidi_browser::test_wait_for_node_existing(launch().await).await;
}
#[tokio::test]
async fn get_inner_text() {
    bidi_browser::test_get_inner_text(launch().await).await;
}
#[tokio::test]
async fn get_text_content() {
    bidi_browser::test_get_text_content(launch().await).await;
}
#[tokio::test]
async fn get_inner_html() {
    bidi_browser::test_get_inner_html(launch().await).await;
}
#[tokio::test]
async fn get_attribute_href() {
    bidi_browser::test_get_attribute_href(launch().await).await;
}
#[tokio::test]
async fn get_children_nodes() {
    bidi_browser::test_get_children_nodes(launch().await).await;
}
#[tokio::test]
async fn node_context_id() {
    bidi_browser::test_node_context_id(launch().await).await;
}
#[tokio::test]
async fn node_is_visible() {
    bidi_browser::test_node_is_visible(launch().await).await;
}
#[tokio::test]
async fn node_scroll_into_view() {
    bidi_browser::test_node_scroll_into_view(launch().await).await;
}
#[tokio::test]
async fn node_get_position() {
    bidi_browser::test_node_get_position(launch().await).await;
}
#[tokio::test]
async fn node_mouse_move() {
    bidi_browser::test_node_mouse_move(launch().await).await;
}
#[tokio::test]
async fn node_mouse_click() {
    bidi_browser::test_node_mouse_click(launch().await).await;
}
#[tokio::test]
async fn mouse_move_to_point() {
    bidi_browser::test_mouse_move_to_point(launch().await).await;
}
#[tokio::test]
async fn mouse_click_at_point() {
    bidi_browser::test_mouse_click_at_point(launch().await).await;
}
#[tokio::test]
async fn mouse_down_and_up() {
    bidi_browser::test_mouse_down_and_up(launch().await).await;
}
#[tokio::test]
async fn keyboard_type_text() {
    bidi_browser::test_keyboard_type_text(launch().await).await;
}
#[tokio::test]
async fn keyboard_press_key() {
    bidi_browser::test_keyboard_press_key(launch().await).await;
}
#[tokio::test]
async fn keyboard_down_and_up() {
    bidi_browser::test_keyboard_down_and_up(launch().await).await;
}
#[tokio::test]
async fn evaluate_script_simple() {
    bidi_browser::test_evaluate_script_simple(launch().await).await;
}
#[tokio::test]
async fn evaluate_script_returns_string() {
    bidi_browser::test_evaluate_script_returns_string(launch().await).await;
}
#[tokio::test]
async fn evaluate_script_returns_number() {
    bidi_browser::test_evaluate_script_returns_number(launch().await).await;
}
#[tokio::test]
async fn create_new_context() {
    bidi_browser::test_create_new_context(launch().await).await;
}
#[tokio::test]
async fn get_active_context_id() {
    bidi_browser::test_get_active_context_id(launch().await).await;
}
#[tokio::test]
async fn screenshot_returns_base64() {
    bidi_browser::test_screenshot_returns_base64(launch().await).await;
}
#[tokio::test]
async fn node_screenshot_returns_base64() {
    bidi_browser::test_node_screenshot_returns_base64(launch().await).await;
}
#[tokio::test]
async fn emulate_timezone() {
    bidi_browser::test_emulate_timezone(launch().await).await;
}
#[tokio::test]
async fn delete_node() {
    bidi_browser::test_delete_node(launch().await).await;
}
#[tokio::test]
async fn end_session_cleanly() {
    bidi_browser::test_end_session_cleanly(launch().await).await;
}

// ── CDP tests ─────────────────────────────────────────────────────────────────
#[tokio::test]
async fn cdp_navigate_to_url() {
    cdp_browser::test_navigate_to_url(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_navigate_returns_frame_or_loader() {
    cdp_browser::test_navigate_returns_frame_or_loader(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_navigate_multiple_pages() {
    cdp_browser::test_navigate_multiple_pages(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_get_accessible_nodes_not_empty() {
    cdp_browser::test_get_accessible_nodes_not_empty(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_get_accessible_nodes_squash_lte_full() {
    cdp_browser::test_get_accessible_nodes_squash_lte_full(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_get_accessible_nodes_squash_no_blank_generic() {
    cdp_browser::test_get_accessible_nodes_squash_no_blank_generic(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_fetch_node_by_node_id() {
    cdp_browser::test_fetch_node_by_node_id(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_fetch_node_with_depth_returns_children() {
    cdp_browser::test_fetch_node_with_depth_returns_children(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_emulate_device_metrics() {
    cdp_browser::test_emulate_device_metrics(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_create_tab() {
    cdp_browser::test_create_tab(launch_cdp().await).await;
}
#[tokio::test]
async fn cdp_preload_script_add_runs_and_remove() {
    cdp_browser::test_preload_script_add_runs_and_remove(launch_cdp().await).await;
}
