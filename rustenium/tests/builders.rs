use rustenium::input::{
    KeyPressOptionsBuilder, KeyboardTypeOptionsBuilder, DelayRange,
    MouseClickOptionsBuilder, MouseMoveOptionsBuilder, MouseOptionsBuilder,
    MouseWheelOptionsBuilder, MouseButton,
    ScrollOptionsBuilder, SwipeOptionsBuilder, TouchMoveOptionsBuilder,
};

// ── Mouse option builders ─────────────────────────────────────────────────────

#[test]
fn mouse_move_options_builder_sets_fields() {
    let opts = MouseMoveOptionsBuilder::default()
        .steps(10)
        .build();
    assert_eq!(opts.steps, Some(10));
    assert!(opts.origin.is_none());
}

#[test]
fn mouse_move_options_builder_default_is_empty() {
    let opts = MouseMoveOptionsBuilder::default().build();
    assert!(opts.steps.is_none());
    assert!(opts.origin.is_none());
}

#[test]
fn mouse_click_options_builder_sets_fields() {
    let opts = MouseClickOptionsBuilder::default()
        .button(MouseButton::Right)
        .count(2)
        .delay(50)
        .build();
    assert_eq!(opts.button, Some(MouseButton::Right));
    assert_eq!(opts.count, Some(2));
    assert_eq!(opts.delay, Some(50));
}

#[test]
fn mouse_click_options_builder_partial() {
    let opts = MouseClickOptionsBuilder::default().count(3).build();
    assert_eq!(opts.count, Some(3));
    assert!(opts.button.is_none());
    assert!(opts.delay.is_none());
}

#[test]
fn mouse_options_builder_sets_button() {
    let opts = MouseOptionsBuilder::default().button(MouseButton::Middle).build();
    assert_eq!(opts.button, Some(MouseButton::Middle));
}

#[test]
fn mouse_wheel_options_builder_sets_deltas() {
    let opts = MouseWheelOptionsBuilder::default().delta_x(20).delta_y(-40).build();
    assert_eq!(opts.delta_x, Some(20));
    assert_eq!(opts.delta_y, Some(-40));
}

#[test]
fn mouse_wheel_options_builder_delta_x_only() {
    let opts = MouseWheelOptionsBuilder::default().delta_x(15).build();
    assert_eq!(opts.delta_x, Some(15));
    assert!(opts.delta_y.is_none());
}

// ── Touch option builders ─────────────────────────────────────────────────────

#[test]
fn swipe_options_builder_sets_duration() {
    let opts = SwipeOptionsBuilder::default().duration_ms(500).build();
    assert_eq!(opts.duration_ms, Some(500));
}

#[test]
fn swipe_options_builder_default_empty() {
    let opts = SwipeOptionsBuilder::default().build();
    assert!(opts.duration_ms.is_none());
}

#[test]
fn scroll_options_builder_sets_duration() {
    let opts = ScrollOptionsBuilder::default().duration_ms(300).build();
    assert_eq!(opts.duration_ms, Some(300));
}

#[test]
fn scroll_options_builder_default_empty() {
    let opts = ScrollOptionsBuilder::default().build();
    assert!(opts.duration_ms.is_none());
}

#[test]
fn touch_move_options_builder_default_empty() {
    let opts = TouchMoveOptionsBuilder::default().build();
    assert!(opts.origin.is_none());
}

// ── Keyboard option builders ──────────────────────────────────────────────────

#[test]
fn delay_range_new_valid() {
    let range = DelayRange::new(10, 50).unwrap();
    assert_eq!(range.min, 10);
    assert_eq!(range.max, 50);
}

#[test]
fn delay_range_new_allows_zero_min() {
    let range = DelayRange::new(0, 100).unwrap();
    assert_eq!(range.min, 0);
    assert_eq!(range.max, 100);
}

#[test]
fn delay_range_new_rejects_zero_max() {
    assert!(DelayRange::new(0, 0).is_none());
}

#[test]
fn delay_range_new_rejects_min_gt_max() {
    assert!(DelayRange::new(200, 100).is_none());
}

#[test]
fn delay_range_equal_min_max_allowed() {
    let range = DelayRange::new(50, 50).unwrap();
    assert_eq!(range.min, 50);
    assert_eq!(range.max, 50);
}

#[test]
fn key_press_options_builder_sets_delay() {
    let opts = KeyPressOptionsBuilder::default().delay(30, 80).build();
    let delay = opts.delay.unwrap();
    assert_eq!(delay.min, 30);
    assert_eq!(delay.max, 80);
}

#[test]
fn key_press_options_builder_invalid_delay_becomes_none() {
    let opts = KeyPressOptionsBuilder::default().delay(100, 50).build();
    assert!(opts.delay.is_none());
}

#[test]
fn key_press_options_builder_default_no_delay() {
    let opts = KeyPressOptionsBuilder::default().build();
    assert!(opts.delay.is_none());
}

#[test]
fn keyboard_type_options_builder_sets_delay_and_gap() {
    let opts = KeyboardTypeOptionsBuilder::default()
        .delay(60, 140)
        .gap_multiplier(1.5)
        .build();
    let delay = opts.delay.unwrap();
    assert_eq!(delay.min, 60);
    assert_eq!(delay.max, 140);
    assert!((opts.gap_multiplier - 1.5).abs() < f64::EPSILON);
}

#[test]
fn keyboard_type_options_builder_default_gap_multiplier() {
    let opts = KeyboardTypeOptionsBuilder::default().build();
    assert!((opts.gap_multiplier - 1.2).abs() < f64::EPSILON);
    assert!(opts.delay.is_none());
}

#[test]
fn keyboard_type_options_default_matches_builder_default() {
    use rustenium::input::KeyboardTypeOptions;
    let direct = KeyboardTypeOptions::default();
    let built = KeyboardTypeOptionsBuilder::default().build();
    assert_eq!(direct.delay.is_none(), built.delay.is_none());
    assert!((direct.gap_multiplier - built.gap_multiplier).abs() < f64::EPSILON);
}
