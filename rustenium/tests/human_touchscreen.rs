use rustenium::input::{
    Point, generate_trajectory, generate_durations, random_curve_params, CurveParams,
};

#[test]
fn trajectory_for_swipe_produces_points() {
    let from = Point { x: 200.0, y: 600.0 };
    let to = Point { x: 200.0, y: 200.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
}

#[test]
fn trajectory_for_short_scroll() {
    let from = Point { x: 200.0, y: 400.0 };
    let to = Point { x: 200.0, y: 380.0 };
    let params = CurveParams {
        offset_boundary_x: 5.0,
        offset_boundary_y: 5.0,
        knots_count: 1,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: |t| t,
        target_points: 10,
    };
    let traj = generate_trajectory(from, to, &params);
    assert_eq!(traj.len(), 10);
}

#[test]
fn trajectory_for_long_scroll() {
    let from = Point { x: 200.0, y: 700.0 };
    let to = Point { x: 200.0, y: 50.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
    let durations = generate_durations(traj.len(), 1.2, (0.004, 0.025));
    let sum: f64 = durations.iter().sum();
    assert!((sum - 1.2).abs() < 1e-9);
}

#[test]
fn vertical_swipe_trajectory_stays_reasonable() {
    let from = Point { x: 360.0, y: 600.0 };
    let to = Point { x: 360.0, y: 100.0 };
    let params = CurveParams {
        offset_boundary_x: 20.0,
        offset_boundary_y: 20.0,
        knots_count: 2,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: |t| t,
        target_points: 30,
    };
    let traj = generate_trajectory(from, to, &params);
    assert_eq!(traj.len(), 30);
    for pt in &traj {
        assert!(pt.x > 300.0 && pt.x < 420.0, "x should stay near center, got {}", pt.x);
    }
}
