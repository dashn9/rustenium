use rustenium::browsers::{FirefoxConfig, FirefoxLaunchMode};

#[test]
fn default_config() {
    let config = FirefoxConfig::default();
    assert!(config.host.is_none());
    assert!(config.proxy.is_none());
    assert!(config.remote_debugging_port.is_none());
    assert!(config.firefox_executable_path.is_none());
    assert!(config.profile_dir.is_none());
    assert!(config.browser_flags.is_none());
    assert!(matches!(
        config.launch_mode,
        FirefoxLaunchMode::SpawnAndAttach
    ));
}

#[test]
fn config_with_custom_values() {
    let config = FirefoxConfig {
        host: Some("127.0.0.1".to_string()),
        remote_debugging_port: Some(2828),
        firefox_executable_path: Some("/usr/bin/firefox".to_string()),
        profile_dir: Some("/tmp/ff-profile".to_string()),
        browser_flags: Some(vec!["-headless".to_string()]),
        launch_mode: FirefoxLaunchMode::Remote(9999),
        ..Default::default()
    };

    assert_eq!(config.host.as_deref(), Some("127.0.0.1"));
    assert_eq!(config.remote_debugging_port, Some(2828));
    assert_eq!(
        config.firefox_executable_path.as_deref(),
        Some("/usr/bin/firefox")
    );
    assert_eq!(config.profile_dir.as_deref(), Some("/tmp/ff-profile"));
    assert_eq!(
        config.browser_flags.as_ref().unwrap(),
        &["-headless".to_string()]
    );
    assert!(matches!(
        config.launch_mode,
        FirefoxLaunchMode::Remote(9999)
    ));
}

#[test]
fn launch_mode_default_is_spawn_and_attach() {
    assert!(matches!(
        FirefoxLaunchMode::default(),
        FirefoxLaunchMode::SpawnAndAttach
    ));
}

#[test]
fn launch_mode_remote_stores_port() {
    let mode = FirefoxLaunchMode::Remote(12345);
    match mode {
        FirefoxLaunchMode::Remote(port) => assert_eq!(port, 12345),
        _ => panic!("expected Remote variant"),
    }
}

#[test]
fn config_clone_preserves_fields() {
    let config = FirefoxConfig {
        remote_debugging_port: Some(7777),
        browser_flags: Some(vec!["-headless".into(), "-private".into()]),
        ..Default::default()
    };
    let cloned = config.clone();
    assert_eq!(cloned.remote_debugging_port, Some(7777));
    assert_eq!(cloned.browser_flags.as_ref().unwrap().len(), 2);
}
