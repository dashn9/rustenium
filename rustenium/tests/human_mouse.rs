use rustenium::input::{
    Point, generate_trajectory, generate_durations, random_curve_params, CurveParams,
};

#[test]
fn random_curve_params_ranges() {
    let from = Point { x: 0.0, y: 0.0 };
    let to = Point { x: 500.0, y: 300.0 };
    for _ in 0..20 {
        let p = random_curve_params(from, to);
        assert!(p.knots_count >= 1 && p.knots_count <= 10);
        assert!(p.target_points >= 35 && p.target_points <= 80);
        assert!(p.distortion_frequency >= 0.25 && p.distortion_frequency <= 0.70);
        assert!(p.distortion_mean >= 0.80 && p.distortion_mean <= 1.10);
        assert!(p.distortion_stdev >= 0.85 && p.distortion_stdev <= 1.10);
        assert!(p.offset_boundary_x >= 20.0 && p.offset_boundary_x < 100.0);
        assert!(p.offset_boundary_y >= 20.0 && p.offset_boundary_y < 100.0);
    }
}

#[test]
fn generate_trajectory_produces_points() {
    let from = Point { x: 10.0, y: 20.0 };
    let to = Point { x: 300.0, y: 400.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2, "trajectory should have at least 2 points");
}

#[test]
fn generate_trajectory_with_many_knots_still_valid() {
    let from = Point { x: 0.0, y: 0.0 };
    let to = Point { x: 100.0, y: 100.0 };
    let params = CurveParams {
        offset_boundary_x: 50.0,
        offset_boundary_y: 50.0,
        knots_count: 10,
        distortion_mean: 1.0,
        distortion_stdev: 1.0,
        distortion_frequency: 0.5,
        tween: |t| t,
        target_points: 60,
    };
    let traj = generate_trajectory(from, to, &params);
    assert_eq!(traj.len(), 60);
}

#[test]
fn generate_trajectory_short_distance() {
    let from = Point { x: 100.0, y: 100.0 };
    let to = Point { x: 102.0, y: 101.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
}

#[test]
fn generate_trajectory_same_point() {
    let p = Point { x: 50.0, y: 50.0 };
    let params = CurveParams {
        offset_boundary_x: 10.0,
        offset_boundary_y: 10.0,
        knots_count: 1,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: |t| t,
        target_points: 10,
    };
    let traj = generate_trajectory(p, p, &params);
    assert!(traj.len() >= 2);
}

#[test]
fn generate_durations_correct_count() {
    let durations = generate_durations(10, 1.0, (0.004, 0.025));
    assert_eq!(durations.len(), 10);
}

#[test]
fn generate_durations_sum_matches_total() {
    let total = 2.5;
    let durations = generate_durations(20, total, (0.004, 0.025));
    let sum: f64 = durations.iter().sum();
    assert!((sum - total).abs() < 1e-9, "sum {} should equal total {}", sum, total);
}

#[test]
fn generate_durations_empty() {
    let durations = generate_durations(0, 1.0, (0.004, 0.025));
    assert!(durations.is_empty());
}

#[test]
fn generate_durations_single() {
    let durations = generate_durations(1, 0.5, (0.004, 0.025));
    assert_eq!(durations.len(), 1);
    assert!((durations[0] - 0.5).abs() < 1e-9);
}

#[test]
fn generate_durations_all_positive() {
    let durations = generate_durations(50, 3.0, (0.004, 0.025));
    for d in &durations {
        assert!(*d > 0.0, "duration should be positive, got {}", d);
    }
}

#[test]
fn trajectory_durations_pair_with_points() {
    let from = Point { x: 100.0, y: 500.0 };
    let to = Point { x: 120.0, y: 100.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    let durations = generate_durations(traj.len(), 0.6, (0.004, 0.025));
    assert_eq!(traj.len(), durations.len());
}
