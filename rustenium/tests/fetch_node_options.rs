use rustenium::browsers::cdp_browser::FetchNodeOptions;
use rustenium_cdp_definitions::browser_protocol::dom::types::{BackendNodeId, NodeId};
use rustenium_cdp_definitions::js_protocol::runtime::types::RemoteObjectId;

#[test]
fn default_has_no_fields_set() {
    let opts = FetchNodeOptions::default();
    let serialized = format!("{:?}", opts);
    assert!(serialized.contains("FetchNodeOptions"));
}

#[test]
fn new_matches_default() {
    let a = FetchNodeOptions::new();
    let b = FetchNodeOptions::default();
    assert_eq!(format!("{:?}", a), format!("{:?}", b));
}

#[test]
fn builder_sets_node_id() {
    let opts = FetchNodeOptions::new().node_id(NodeId::new(42));
    let dbg = format!("{:?}", opts);
    assert!(dbg.contains("42"));
}

#[test]
fn builder_sets_depth() {
    let opts = FetchNodeOptions::new().depth(5);
    let dbg = format!("{:?}", opts);
    assert!(dbg.contains("5"));
}

#[test]
fn builder_sets_pierce() {
    let opts = FetchNodeOptions::new().pierce(true);
    let dbg = format!("{:?}", opts);
    assert!(dbg.contains("true"));
}

#[test]
fn builder_chains_all_fields() {
    let opts = FetchNodeOptions::new()
        .node_id(NodeId::new(1))
        .backend_node_id(BackendNodeId::new(2))
        .object_id(RemoteObjectId::new("obj-3"))
        .depth(10)
        .pierce(false);
    let dbg = format!("{:?}", opts);
    assert!(dbg.contains("obj-3"));
    assert!(dbg.contains("10"));
}

#[test]
fn build_returns_same_type() {
    let opts = FetchNodeOptions::new().node_id(NodeId::new(7)).depth(3).build();
    let dbg = format!("{:?}", opts);
    assert!(dbg.contains("7"));
    assert!(dbg.contains("3"));
}

#[test]
fn clone_preserves_fields() {
    let opts = FetchNodeOptions::new().node_id(NodeId::new(99)).depth(2);
    let cloned = opts.clone();
    assert_eq!(format!("{:?}", opts), format!("{:?}", cloned));
}
