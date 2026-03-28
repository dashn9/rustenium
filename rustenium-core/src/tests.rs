mod transport_tests {
    use crate::transport::{ConnectionTransportConfig, ConnectionTransportProtocol};

    #[test]
    fn default_config() {
        let config = ConnectionTransportConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 0);
        assert_eq!(config.path, "session".to_string());
    }

    #[test]
    fn full_endpoint_formats_correctly() {
        let configs_and_expected = vec![
            (
                ConnectionTransportConfig {
                    protocol: ConnectionTransportProtocol::Ws,
                    host: "127.0.0.1".to_string(),
                    port: 9222,
                    path: "session".to_string(),
                },
                "ws://127.0.0.1:9222/session",
            ),
            (
                ConnectionTransportConfig {
                    protocol: ConnectionTransportProtocol::Wss,
                    host: "example.com".to_string(),
                    port: 443,
                    path: "/custom/path".to_string(),
                },
                "wss://example.com:443/custom/path",
            ),
            (
                ConnectionTransportConfig {
                    protocol: ConnectionTransportProtocol::Http,
                    host: "0.0.0.0".to_string(),
                    port: 8080,
                    path: "api".to_string(),
                },
                "http://0.0.0.0:8080/api",
            ),
        ];

        for (config, expected) in configs_and_expected {
            assert_eq!(config.full_endpoint(), expected);
        }
    }

    #[test]
    fn path_leading_slash_handling() {
        let no_slash = ConnectionTransportConfig { path: "no-slash".to_string(), ..Default::default() };
        let with_slash = ConnectionTransportConfig { path: "/already-slashed".to_string(), ..Default::default() };

        assert_eq!(no_slash.path(), "/no-slash");
        assert_eq!(with_slash.path(), "/already-slashed");
    }

    #[test]
    fn host_port_formatting() {
        let config = ConnectionTransportConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            ..Default::default()
        };
        assert_eq!(config.host_port(), "0.0.0.0:8080");
    }

    #[test]
    fn protocol_display() {
        let cases: Vec<(ConnectionTransportProtocol, &str)> = vec![
            (ConnectionTransportProtocol::Http, "http"),
            (ConnectionTransportProtocol::Https, "https"),
            (ConnectionTransportProtocol::Ws, "ws"),
            (ConnectionTransportProtocol::Wss, "wss"),
        ];
        for (proto, expected) in cases {
            assert_eq!(format!("{}", proto), expected);
        }
    }
}

mod connection_tests {
    use crate::connection::find_free_port;

    #[test]
    fn find_free_port_returns_valid_ports() {
        let ports: Vec<u16> = (0..5).map(|_| find_free_port().unwrap()).collect();
        for port in &ports {
            assert!(*port > 0);
        }
    }
}

mod error_tests {
    use crate::error::*;

    #[test]
    fn error_display_messages() {
        let cases: Vec<(Box<dyn std::error::Error>, &str)> = vec![
            (
                Box::new(SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)),
                "receive response",
            ),
            (
                Box::new(CommandResultError::InvalidResultTypeError(serde_json::json!({}))),
                "Invalid Result",
            ),
            (
                Box::new(PostDataError::NoPostData),
                "POST data",
            ),
            (
                Box::new(PostDataError::NotJsonObject),
                "not a JSON object",
            ),
        ];

        for (err, expected_substr) in cases {
            assert!(
                format!("{}", err).contains(expected_substr),
                "Expected '{}' to contain '{}'", err, expected_substr
            );
        }
    }

    #[test]
    fn post_data_error_from_serde() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = PostDataError::from(json_err);
        assert!(matches!(err, PostDataError::JsonParseError(_)));
    }
}

mod listener_tests {
    use crate::listeners::{CommandResponseState, Listener, CommandResponseListener, EventListener};
    use rustenium_bidi_definitions::base::{CommandResponse, SuccessEnum};
    use tokio::sync::mpsc::unbounded_channel;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[test]
    fn command_response_state_serialization_roundtrip() {
        let response = CommandResponse {
            r#type: SuccessEnum::Success,
            id: 42,
            result: serde_json::json!({"status": "ok"}),
            extensible: HashMap::new(),
        };
        let state = CommandResponseState::Success(response);
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: CommandResponseState = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CommandResponseState::Success(r) if r.id == 42));
    }

    #[test]
    fn event_listener_new_has_empty_listeners() {
        let el = EventListener::new();
        assert!(el.listeners.try_lock().unwrap().is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn listener_routing() {
        // Test valid command, error response, and invalid JSON concurrently
        let (valid_result, error_result, skip_result) = tokio::join!(
            route_valid_command(),
            route_error_response(),
            skip_invalid_then_route_valid(),
        );

        assert!(matches!(valid_result, CommandResponseState::Success(r) if r.id == 1));
        assert!(matches!(error_result, CommandResponseState::Error(_)));
        assert!(matches!(skip_result, CommandResponseState::Success(r) if r.id == 99));
    }

    async fn route_valid_command() -> CommandResponseState {
        let (raw_tx, raw_rx) = unbounded_channel::<String>();
        let (cmd_tx, mut cmd_rx) = unbounded_channel::<CommandResponseState>();
        let (evt_tx, _evt_rx) = unbounded_channel();

        Listener::new(raw_rx, cmd_tx, evt_tx).start();

        raw_tx.send(serde_json::json!({
            "type": "success", "id": 1, "result": {}
        }).to_string()).unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(2), cmd_rx.recv())
            .await.unwrap().unwrap()
    }

    async fn route_error_response() -> CommandResponseState {
        let (raw_tx, raw_rx) = unbounded_channel::<String>();
        let (cmd_tx, mut cmd_rx) = unbounded_channel::<CommandResponseState>();
        let (evt_tx, _evt_rx) = unbounded_channel();

        Listener::new(raw_rx, cmd_tx, evt_tx).start();

        raw_tx.send(serde_json::json!({
            "type": "error", "id": 5, "error": "unknown command",
            "message": "bad", "stacktrace": ""
        }).to_string()).unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(2), cmd_rx.recv())
            .await.unwrap().unwrap()
    }

    async fn skip_invalid_then_route_valid() -> CommandResponseState {
        let (raw_tx, raw_rx) = unbounded_channel::<String>();
        let (cmd_tx, mut cmd_rx) = unbounded_channel::<CommandResponseState>();
        let (evt_tx, _evt_rx) = unbounded_channel();

        Listener::new(raw_rx, cmd_tx, evt_tx).start();

        raw_tx.send("not valid json".to_string()).unwrap();
        raw_tx.send(serde_json::json!({
            "type": "success", "id": 99, "result": {}
        }).to_string()).unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(2), cmd_rx.recv())
            .await.unwrap().unwrap()
    }

    #[tokio::test]
    async fn command_response_listener_dispatches_to_subscription() {
        let (cmd_tx, cmd_rx) = unbounded_channel::<CommandResponseState>();
        let subscriptions = Arc::new(Mutex::new(HashMap::new()));

        let (oneshot_tx, oneshot_rx) = tokio::sync::oneshot::channel();
        subscriptions.lock().await.insert(7u64, oneshot_tx);

        CommandResponseListener::new(cmd_rx, subscriptions).start();

        cmd_tx.send(CommandResponseState::Success(CommandResponse {
            r#type: SuccessEnum::Success,
            id: 7,
            result: serde_json::json!({"done": true}),
            extensible: HashMap::new(),
        })).unwrap();

        let result = tokio::time::timeout(std::time::Duration::from_secs(2), oneshot_rx)
            .await.unwrap().unwrap();
        assert!(matches!(result, CommandResponseState::Success(r) if r.id == 7));
    }
}

mod process_tests {
    use crate::process::Process;

    #[tokio::test]
    async fn process_create_and_drop() {
        #[cfg(unix)]
        let proc = Process::create("echo", vec!["hello".to_string()]);
        #[cfg(windows)]
        let proc = Process::create("cmd", vec!["/C".to_string(), "echo".to_string(), "hello".to_string()]);

        drop(proc);
    }

    #[tokio::test]
    async fn process_create_with_empty_args() {
        #[cfg(unix)]
        let proc = Process::create("true", Vec::<String>::new());
        #[cfg(windows)]
        let proc = Process::create("cmd", vec!["/C".to_string()]);

        drop(proc);
    }
}
