use rustenium::nodes::{NodePosition, NodeScreenShotOptions, NodeType};

// ── NodeType ──────────────────────────────────────────────────────────────────

#[test]
fn node_type_as_u16_matches_repr() {
    assert_eq!(NodeType::Element.as_u16(), 1);
    assert_eq!(NodeType::Attribute.as_u16(), 2);
    assert_eq!(NodeType::Text.as_u16(), 3);
    assert_eq!(NodeType::CDataSection.as_u16(), 4);
    assert_eq!(NodeType::ProcessingInstruction.as_u16(), 7);
    assert_eq!(NodeType::Comment.as_u16(), 8);
    assert_eq!(NodeType::Document.as_u16(), 9);
    assert_eq!(NodeType::DocumentType.as_u16(), 10);
    assert_eq!(NodeType::DocumentFragment.as_u16(), 11);
}

#[test]
fn node_type_from_u16_known_values() {
    assert_eq!(NodeType::from_u16(1), Some(NodeType::Element));
    assert_eq!(NodeType::from_u16(2), Some(NodeType::Attribute));
    assert_eq!(NodeType::from_u16(3), Some(NodeType::Text));
    assert_eq!(NodeType::from_u16(4), Some(NodeType::CDataSection));
    assert_eq!(NodeType::from_u16(7), Some(NodeType::ProcessingInstruction));
    assert_eq!(NodeType::from_u16(8), Some(NodeType::Comment));
    assert_eq!(NodeType::from_u16(9), Some(NodeType::Document));
    assert_eq!(NodeType::from_u16(10), Some(NodeType::DocumentType));
    assert_eq!(NodeType::from_u16(11), Some(NodeType::DocumentFragment));
}

#[test]
fn node_type_from_u16_unknown_returns_none() {
    assert_eq!(NodeType::from_u16(0), None);
    assert_eq!(NodeType::from_u16(5), None);
    assert_eq!(NodeType::from_u16(6), None);
    assert_eq!(NodeType::from_u16(12), None);
    assert_eq!(NodeType::from_u16(100), None);
    assert_eq!(NodeType::from_u16(u16::MAX), None);
}

#[test]
fn node_type_round_trip_from_as_u16() {
    let values = [1u16, 2, 3, 4, 7, 8, 9, 10, 11];
    for v in values {
        let t = NodeType::from_u16(v).unwrap();
        assert_eq!(t.as_u16(), v);
    }
}

#[test]
fn node_type_is_element() {
    assert!(NodeType::Element.is_element());
    assert!(!NodeType::Text.is_element());
    assert!(!NodeType::Comment.is_element());
    assert!(!NodeType::Document.is_element());
}

#[test]
fn node_type_is_text() {
    assert!(NodeType::Text.is_text());
    assert!(!NodeType::Element.is_text());
    assert!(!NodeType::Comment.is_text());
    assert!(!NodeType::CDataSection.is_text());
}

#[test]
fn node_type_equality_and_copy() {
    let a = NodeType::Element;
    let b = a;
    assert_eq!(a, b);
    assert_ne!(NodeType::Element, NodeType::Text);
}

// ── NodeScreenShotOptions ─────────────────────────────────────────────────────

#[test]
fn node_screenshot_options_default_all_none() {
    let opts = NodeScreenShotOptions::default();
    assert!(opts.origin.is_none());
    assert!(opts.bidi_format.is_none());
    assert!(opts.cdp_format.is_none());
    assert!(opts.quality.is_none());
    assert!(opts.from_surface.is_none());
    assert!(opts.capture_beyond_viewport.is_none());
    assert!(opts.optimize_for_speed.is_none());
    assert!(opts.save_path.is_none());
}

#[test]
fn node_screenshot_options_clone() {
    let opts = NodeScreenShotOptions {
        quality: Some(0.75),
        save_path: Some("/tmp/shot.png".to_string()),
        ..Default::default()
    };
    let cloned = opts.clone();
    assert_eq!(cloned.quality, Some(0.75));
    assert_eq!(cloned.save_path.as_deref(), Some("/tmp/shot.png"));
}

// ── NodePosition ──────────────────────────────────────────────────────────────

#[test]
fn node_position_fields_accessible() {
    let pos = NodePosition {
        x: 100.0,
        y: 200.0,
        scroll_x: 10.0,
        scroll_y: 20.0,
        width: 50.0,
        height: 30.0,
    };
    assert!((pos.x - 100.0).abs() < f64::EPSILON);
    assert!((pos.y - 200.0).abs() < f64::EPSILON);
    assert!((pos.scroll_x - 10.0).abs() < f64::EPSILON);
    assert!((pos.scroll_y - 20.0).abs() < f64::EPSILON);
    assert!((pos.width - 50.0).abs() < f64::EPSILON);
    assert!((pos.height - 30.0).abs() < f64::EPSILON);
}

#[test]
fn node_position_equality() {
    let a = NodePosition {
        x: 1.0,
        y: 2.0,
        scroll_x: 0.0,
        scroll_y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn node_position_deserialize_from_json() {
    let json = serde_json::json!({
        "x": 12.5,
        "y": 34.0,
        "scroll_x": 0.0,
        "scroll_y": 100.0,
        "width": 200.0,
        "height": 80.0,
    });
    let pos: NodePosition = serde_json::from_value(json).unwrap();
    assert!((pos.x - 12.5).abs() < f64::EPSILON);
    assert!((pos.scroll_y - 100.0).abs() < f64::EPSILON);
    assert!((pos.width - 200.0).abs() < f64::EPSILON);
}
