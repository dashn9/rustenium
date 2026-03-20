use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustenium_core::transport::{ConnectionTransportConfig, ConnectionTransportProtocol};
use rustenium_core::find_free_port;
use rustenium_core::CommandResponseState;
use rustenium_bidi_definitions::base::{CommandResponse, ErrorCode, ErrorResponse, ErrorEnum, Message, SuccessEnum};
use std::collections::HashMap;

fn bench_transport_config_formatting(c: &mut Criterion) {
    let config = ConnectionTransportConfig {
        protocol: ConnectionTransportProtocol::Ws,
        host: "127.0.0.1".to_string(),
        port: 9222,
        path: "session",
    };

    c.bench_function("full_endpoint", |b| {
        b.iter(|| black_box(config.full_endpoint()))
    });

    c.bench_function("host_port", |b| {
        b.iter(|| black_box(config.host_port()))
    });

    c.bench_function("path", |b| {
        b.iter(|| black_box(config.path()))
    });
}

fn bench_find_free_port(c: &mut Criterion) {
    c.bench_function("find_free_port", |b| {
        b.iter(|| black_box(find_free_port().unwrap()))
    });
}

fn bench_message_parsing(c: &mut Criterion) {
    let success_json = serde_json::json!({
        "type": "success", "id": 1, "result": {"status": "ok"}
    }).to_string();

    let error_json = serde_json::json!({
        "type": "error", "id": 5, "error": "unknown command",
        "message": "bad", "stacktrace": ""
    }).to_string();

    let event_json = serde_json::json!({
        "type": "event", "method": "browsingContext.load",
        "params": {"context": "ctx-1", "navigation": "nav-1", "timestamp": 12345, "url": "https://example.com"}
    }).to_string();

    let invalid_json = "not valid json at all".to_string();

    c.bench_function("parse_success_message", |b| {
        b.iter(|| black_box(serde_json::from_str::<Message>(&success_json)))
    });

    c.bench_function("parse_error_message", |b| {
        b.iter(|| black_box(serde_json::from_str::<Message>(&error_json)))
    });

    c.bench_function("parse_event_message", |b| {
        b.iter(|| black_box(serde_json::from_str::<Message>(&event_json)))
    });

    c.bench_function("parse_invalid_json", |b| {
        b.iter(|| black_box(serde_json::from_str::<Message>(&invalid_json)))
    });
}

fn bench_command_response_state_serde(c: &mut Criterion) {
    let success_state = CommandResponseState::Success(CommandResponse {
        r#type: SuccessEnum::Success,
        id: 42,
        result: serde_json::json!({"status": "ok", "data": [1, 2, 3]}),
        extensible: HashMap::new(),
    });

    let error_state = CommandResponseState::Error(ErrorResponse {
        r#type: ErrorEnum::Error,
        id: Some(5),
        error: ErrorCode::UnknownCommand,
        message: "bad command".to_string(),
        stacktrace: Some(String::new()),
        extensible: HashMap::new(),
    });

    let success_json = serde_json::to_string(&success_state).unwrap();
    let error_json = serde_json::to_string(&error_state).unwrap();

    c.bench_function("serialize_success_state", |b| {
        b.iter(|| black_box(serde_json::to_string(&success_state).unwrap()))
    });

    c.bench_function("serialize_error_state", |b| {
        b.iter(|| black_box(serde_json::to_string(&error_state).unwrap()))
    });

    c.bench_function("deserialize_success_state", |b| {
        b.iter(|| black_box(serde_json::from_str::<CommandResponseState>(&success_json).unwrap()))
    });

    c.bench_function("deserialize_error_state", |b| {
        b.iter(|| black_box(serde_json::from_str::<CommandResponseState>(&error_json).unwrap()))
    });
}

criterion_group!(
    benches,
    bench_transport_config_formatting,
    bench_find_free_port,
    bench_message_parsing,
    bench_command_response_state_serde,
);
criterion_main!(benches);
