use rustenium::browsers::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};

#[tokio::test]
async fn default_capabilities() {
    let caps = ChromeCapabilities::default();
    assert_eq!(caps.base_capabilities.browser_name.as_deref(), Some("chrome"));
    assert!(caps.base_capabilities.accept_insecure_certs.is_none());
    assert!(caps.base_capabilities.proxy.is_none());
    assert!(caps.chrome_options.args.is_none());
    assert!(caps.chrome_options.binary.is_none());
}

#[tokio::test]
async fn builder_methods() {
    let mut caps = ChromeCapabilities::default();
    caps.accept_insecure_certs(true)
        .browser_version("120")
        .platform_name("linux")
        .binary("/usr/bin/chrome")
        .detach(true)
        .debugger_address("localhost:9222")
        .enable_extension_targets(true)
        .use_automation_extension(false)
        .minidump_path("/tmp/dumps");

    assert_eq!(caps.base_capabilities.accept_insecure_certs, Some(true));
    assert_eq!(caps.base_capabilities.browser_version.as_deref(), Some("120"));
    assert_eq!(caps.base_capabilities.platform_name.as_deref(), Some("linux"));
    assert_eq!(caps.chrome_options.binary.as_deref(), Some("/usr/bin/chrome"));
    assert_eq!(caps.chrome_options.detach, Some(true));
    assert_eq!(caps.chrome_options.debugger_address.as_deref(), Some("localhost:9222"));
    assert_eq!(caps.chrome_options.enable_extension_targets, Some(true));
    assert_eq!(caps.chrome_options.use_automation_extension, Some(false));
}

#[tokio::test]
async fn add_arg_accumulates() {
    let mut caps = ChromeCapabilities::default();
    caps.add_arg("--headless")
        .add_arg("--disable-gpu")
        .add_arg("--no-sandbox");

    let args = caps.chrome_options.args.as_ref().unwrap();
    assert_eq!(args.len(), 3);
    assert!(args.contains(&"--headless".to_string()));
    assert!(args.contains(&"--no-sandbox".to_string()));
}

#[tokio::test]
async fn add_args_batch() {
    let mut caps = ChromeCapabilities::default();
    caps.add_args(["--headless", "--disable-gpu"]);
    assert_eq!(caps.chrome_options.args.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn add_args_extends_existing() {
    let mut caps = ChromeCapabilities::default();
    caps.add_arg("--first");
    caps.add_args(["--second", "--third"]);
    assert_eq!(caps.chrome_options.args.as_ref().unwrap().len(), 3);
}

#[tokio::test]
async fn add_extension_accumulates() {
    let mut caps = ChromeCapabilities::default();
    caps.add_extension("ext1").add_extension("ext2");
    assert_eq!(caps.chrome_options.extensions.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn add_exclude_switch_accumulates() {
    let mut caps = ChromeCapabilities::default();
    caps.add_exclude_switch("enable-automation");
    assert_eq!(caps.chrome_options.exclude_switches.as_ref().unwrap(), &["enable-automation"]);
}

#[tokio::test]
async fn add_pref_accumulates() {
    let mut caps = ChromeCapabilities::default();
    caps.add_pref("download.default_directory", serde_json::json!("/tmp"))
        .add_pref("profile.managed_default_content_settings.images", serde_json::json!(2));
    assert_eq!(caps.chrome_options.prefs.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn add_window_type_accumulates() {
    let mut caps = ChromeCapabilities::default();
    caps.add_window_type("webview");
    assert_eq!(caps.chrome_options.window_types.as_ref().unwrap(), &["webview"]);
}

#[tokio::test]
async fn build_produces_capabilities_request() {
    let mut caps = ChromeCapabilities::default();
    caps.add_arg("--headless").accept_insecure_certs(true);

    let request = caps.build();
    let always_match = request.always_match.unwrap();
    assert_eq!(always_match.accept_insecure_certs, Some(true));

    let chrome_opts = always_match.extensible.get("goog:chromeOptions").unwrap();
    let args = chrome_opts.get("args").unwrap().as_array().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].as_str().unwrap(), "--headless");
}

#[tokio::test]
async fn chrome_options_skip_serializing_none_fields() {
    let opts = ChromeOptions::default();
    let json = serde_json::to_value(&opts).unwrap();
    let obj = json.as_object().unwrap();
    assert!(obj.is_empty(), "Default ChromeOptions should serialize to empty object, got: {:?}", obj);
}

#[tokio::test]
async fn chrome_options_serializes_set_fields() {
    let opts = ChromeOptions {
        args: Some(vec!["--headless".to_string()]),
        binary: Some("/usr/bin/chrome".to_string()),
        detach: Some(true),
        ..Default::default()
    };
    let json = serde_json::to_value(&opts).unwrap();
    assert_eq!(json["args"][0], "--headless");
    assert_eq!(json["binary"], "/usr/bin/chrome");
    assert_eq!(json["detach"], true);
    assert!(json.get("extensions").is_none());
}

#[tokio::test]
async fn perf_logging_prefs_builder() {
    let mut prefs = PerfLoggingPrefs::new();
    prefs.enable_network(true)
        .enable_page(false)
        .trace_categories("devtools.timeline")
        .buffer_usage_reporting_interval(500);

    assert_eq!(prefs.enable_network, Some(true));
    assert_eq!(prefs.enable_page, Some(false));
    assert_eq!(prefs.trace_categories.as_deref(), Some("devtools.timeline"));
    assert_eq!(prefs.buffer_usage_reporting_interval, Some(500));
}

#[tokio::test]
async fn perf_logging_prefs_serialization() {
    let mut prefs = PerfLoggingPrefs::new();
    prefs.enable_network(true);

    let json = serde_json::to_value(&prefs).unwrap();
    assert_eq!(json["enableNetwork"], true);
    assert!(json.get("enablePage").is_none());
}
