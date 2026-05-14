use rustenium::browsers::{FirefoxCapabilities, FirefoxOptions};

#[test]
fn default_capabilities() {
    let caps = FirefoxCapabilities::default();
    assert_eq!(caps.base_capabilities.browser_name.as_deref(), Some("firefox"));
    assert!(caps.base_capabilities.accept_insecure_certs.is_none());
    assert!(caps.base_capabilities.proxy.is_none());
    assert!(caps.firefox_options.args.is_none());
    assert!(caps.firefox_options.binary.is_none());
    assert!(caps.firefox_options.profile.is_none());
    assert!(caps.firefox_options.prefs.is_none());
}

#[test]
fn builder_base_fields() {
    let mut caps = FirefoxCapabilities::default();
    caps.accept_insecure_certs(true)
        .browser_name("firefox-esr")
        .browser_version("115")
        .platform_name("linux");

    assert_eq!(caps.base_capabilities.accept_insecure_certs, Some(true));
    assert_eq!(caps.base_capabilities.browser_name.as_deref(), Some("firefox-esr"));
    assert_eq!(caps.base_capabilities.browser_version.as_deref(), Some("115"));
    assert_eq!(caps.base_capabilities.platform_name.as_deref(), Some("linux"));
}

#[test]
fn builder_firefox_options() {
    let mut caps = FirefoxCapabilities::default();
    caps.binary("/usr/bin/firefox")
        .profile("/tmp/ff-profile")
        .log_level("debug");

    assert_eq!(caps.firefox_options.binary.as_deref(), Some("/usr/bin/firefox"));
    assert_eq!(caps.firefox_options.profile.as_deref(), Some("/tmp/ff-profile"));
    assert_eq!(caps.firefox_options.log.as_ref().unwrap().level, "debug");
}

#[test]
fn add_arg_accumulates() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_arg("-headless")
        .add_arg("-no-remote")
        .add_arg("-private");

    let args = caps.firefox_options.args.as_ref().unwrap();
    assert_eq!(args.len(), 3);
    assert!(args.contains(&"-headless".to_string()));
    assert!(args.contains(&"-private".to_string()));
}

#[test]
fn add_args_batch() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_args(["-headless", "-no-remote"]);
    assert_eq!(caps.firefox_options.args.as_ref().unwrap().len(), 2);
}

#[test]
fn add_args_extends_existing() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_arg("-first");
    caps.add_args(["-second", "-third"]);
    assert_eq!(caps.firefox_options.args.as_ref().unwrap().len(), 3);
}

#[test]
fn add_pref_accumulates() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_pref("dom.disable_beforeunload", serde_json::json!(true))
        .add_pref("browser.startup.page", serde_json::json!(0));
    let prefs = caps.firefox_options.prefs.as_ref().unwrap();
    assert_eq!(prefs.len(), 2);
    assert_eq!(prefs.get("browser.startup.page").unwrap(), &serde_json::json!(0));
}

#[test]
fn add_env_accumulates() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_env("MOZ_LOG", "sync:5")
        .add_env("MOZ_LOG_FILE", "/tmp/ff.log");
    let env = caps.firefox_options.env.as_ref().unwrap();
    assert_eq!(env.len(), 2);
    assert_eq!(env.get("MOZ_LOG").map(String::as_str), Some("sync:5"));
}

#[test]
fn build_injects_web_socket_url_and_moz_options() {
    let mut caps = FirefoxCapabilities::default();
    caps.add_arg("-headless").accept_insecure_certs(true);

    let request = caps.build();
    let always_match = request.always_match.unwrap();
    assert_eq!(always_match.accept_insecure_certs, Some(true));

    let ws_url = always_match.extensible.get("webSocketUrl").unwrap();
    assert_eq!(ws_url, &serde_json::Value::Bool(true));

    let moz_opts = always_match.extensible.get("moz:firefoxOptions").unwrap();
    let args = moz_opts.get("args").unwrap().as_array().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].as_str().unwrap(), "-headless");
}

#[test]
fn firefox_options_skip_serializing_none_fields() {
    let opts = FirefoxOptions::default();
    let json = serde_json::to_value(&opts).unwrap();
    let obj = json.as_object().unwrap();
    assert!(obj.is_empty(), "Default FirefoxOptions should serialize to empty object, got: {:?}", obj);
}

#[test]
fn firefox_options_serializes_set_fields() {
    let opts = FirefoxOptions {
        args: Some(vec!["-headless".to_string()]),
        binary: Some("/usr/bin/firefox".to_string()),
        profile: Some("/tmp/profile".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_value(&opts).unwrap();
    assert_eq!(json["args"][0], "-headless");
    assert_eq!(json["binary"], "/usr/bin/firefox");
    assert_eq!(json["profile"], "/tmp/profile");
    assert!(json.get("prefs").is_none());
}
