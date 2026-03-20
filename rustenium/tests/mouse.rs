use rustenium::input::{Point, MouseButton, MouseClickOptions, MouseMoveOptions, MouseOptions, MouseWheelOptions};

#[tokio::test]
async fn point_default_is_origin() {
    let p = Point::default();
    assert!((p.x).abs() < f64::EPSILON);
    assert!((p.y).abs() < f64::EPSILON);
}

#[tokio::test]
async fn mouse_button_values() {
    assert_eq!(MouseButton::Left as u64, 0);
    assert_eq!(MouseButton::Middle as u64, 1);
    assert_eq!(MouseButton::Right as u64, 2);
    assert_eq!(MouseButton::Back as u64, 3);
    assert_eq!(MouseButton::Forward as u64, 4);
}

#[tokio::test]
async fn mouse_button_equality() {
    assert_eq!(MouseButton::Left, MouseButton::Left);
    assert_ne!(MouseButton::Left, MouseButton::Right);
}

#[tokio::test]
async fn mouse_move_options_defaults() {
    let opts = MouseMoveOptions::default();
    assert!(opts.steps.is_none());
    assert!(opts.origin.is_none());
}

#[tokio::test]
async fn mouse_click_options_defaults() {
    let opts = MouseClickOptions::default();
    assert!(opts.button.is_none());
    assert!(opts.count.is_none());
    assert!(opts.delay.is_none());
}

#[tokio::test]
async fn mouse_options_defaults() {
    let opts = MouseOptions::default();
    assert!(opts.button.is_none());
}

#[tokio::test]
async fn mouse_wheel_options_defaults() {
    let opts = MouseWheelOptions::default();
    assert!(opts.delta_x.is_none());
    assert!(opts.delta_y.is_none());
}
