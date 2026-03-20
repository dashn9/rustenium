use rustenium::browsers::ChromeConfig;

#[tokio::test]
async fn default_config() {
    let config = ChromeConfig::default();
    assert_eq!(config.driver_executable_path, "");
    assert!(config.host.is_none());
    assert!(config.port.is_none());
    assert!(!config.sandbox);
    assert!(config.proxy.is_none());
    assert!(config.remote_debugging_port.is_none());
    assert!(config.chrome_executable_path.is_none());
    assert!(config.user_data_dir.is_none());
    assert!(config.browser_flags.is_none());
}

#[tokio::test]
async fn config_with_custom_values() {
    let config = ChromeConfig {
        driver_executable_path: "/usr/bin/chromedriver".to_string(),
        host: Some("0.0.0.0".to_string()),
        port: Some(4444),
        sandbox: true,
        remote_debugging_port: Some(0),
        chrome_executable_path: Some("/usr/bin/google-chrome".to_string()),
        user_data_dir: Some("/tmp/chrome-profile".to_string()),
        browser_flags: Some(vec!["--headless".to_string()]),
        ..Default::default()
    };

    assert_eq!(config.driver_executable_path, "/usr/bin/chromedriver");
    assert_eq!(config.host.as_deref(), Some("0.0.0.0"));
    assert_eq!(config.port, Some(4444));
    assert!(config.sandbox);
    assert_eq!(config.remote_debugging_port, Some(0));
    assert_eq!(config.chrome_executable_path.as_deref(), Some("/usr/bin/google-chrome"));
    assert_eq!(config.user_data_dir.as_deref(), Some("/tmp/chrome-profile"));
    assert_eq!(config.browser_flags.as_ref().unwrap(), &["--headless".to_string()]);
}
