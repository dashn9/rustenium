use rustenium::input::{
    Point, generate_trajectory, generate_durations, random_curve_params,
    weighted_pick, CurveParams, linear,
};

#[tokio::test]
async fn generate_durations_correct_count() {
    let durations = generate_durations(10, 1.0, (0.004, 0.025));
    assert_eq!(durations.len(), 10);
}

#[tokio::test]
async fn generate_durations_sum_matches_total() {
    let total = 2.5;
    let durations = generate_durations(20, total, (0.004, 0.025));
    let sum: f64 = durations.iter().sum();
    assert!((sum - total).abs() < 1e-9, "sum {} should equal total {}", sum, total);
}

#[tokio::test]
async fn generate_durations_empty() {
    let durations = generate_durations(0, 1.0, (0.004, 0.025));
    assert!(durations.is_empty());
}

#[tokio::test]
async fn generate_durations_single() {
    let durations = generate_durations(1, 0.5, (0.004, 0.025));
    assert_eq!(durations.len(), 1);
    assert!((durations[0] - 0.5).abs() < 1e-9);
}

#[tokio::test]
async fn generate_durations_all_positive() {
    let durations = generate_durations(50, 3.0, (0.004, 0.025));
    for d in &durations {
        assert!(*d > 0.0, "duration should be positive, got {}", d);
    }
}

#[tokio::test]
async fn zone_weights_favor_bottom_center() {
    // Zone 7 = bottom-center, should be picked most often with weight 0.45
    let mut rng = rand::rng();
    let zone_weights: &[(usize, f64)] = &[
        (0, 0.02), (1, 0.05), (2, 0.02),
        (3, 0.05), (4, 0.15), (5, 0.05),
        (6, 0.07), (7, 0.45), (8, 0.07),
    ];
    let mut counts = [0usize; 9];
    let n = 10_000;
    for _ in 0..n {
        let zone = weighted_pick(&mut rng, zone_weights);
        counts[zone] += 1;
    }
    // BC (zone 7) should be the most picked
    let bc = counts[7];
    for (i, &c) in counts.iter().enumerate() {
        if i != 7 {
            assert!(bc > c, "BC ({}) should beat zone {} ({})", bc, i, c);
        }
    }
    // BC should be roughly 40-50% of picks
    let pct = bc as f64 / n as f64 * 100.0;
    assert!(pct > 35.0 && pct < 55.0, "BC should be ~45%, got {:.1}%", pct);
}

#[tokio::test]
async fn zone_weights_corners_are_rare() {
    let mut rng = rand::rng();
    let zone_weights: &[(usize, f64)] = &[
        (0, 0.02), (1, 0.05), (2, 0.02),
        (3, 0.05), (4, 0.15), (5, 0.05),
        (6, 0.07), (7, 0.45), (8, 0.07),
    ];
    let mut counts = [0usize; 9];
    let n = 10_000;
    for _ in 0..n {
        counts[weighted_pick(&mut rng, zone_weights)] += 1;
    }
    // TL and TR (zones 0,2) should each be <5%
    let tl_pct = counts[0] as f64 / n as f64 * 100.0;
    let tr_pct = counts[2] as f64 / n as f64 * 100.0;
    assert!(tl_pct < 5.0, "TL should be <5%, got {:.1}%", tl_pct);
    assert!(tr_pct < 5.0, "TR should be <5%, got {:.1}%", tr_pct);
}

#[tokio::test]
async fn trajectory_for_swipe_produces_points() {
    let from = Point { x: 200.0, y: 600.0 };
    let to = Point { x: 200.0, y: 200.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
}

#[tokio::test]
async fn trajectory_durations_pair_with_points() {
    let from = Point { x: 100.0, y: 500.0 };
    let to = Point { x: 120.0, y: 100.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    let durations = generate_durations(traj.len(), 0.6, (0.004, 0.025));
    assert_eq!(traj.len(), durations.len());
}

#[tokio::test]
async fn trajectory_for_short_scroll() {
    let from = Point { x: 200.0, y: 400.0 };
    let to = Point { x: 200.0, y: 380.0 };
    let params = CurveParams {
        offset_boundary_x: 5.0,
        offset_boundary_y: 5.0,
        knots_count: 1,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: linear,
        target_points: 10,
    };
    let traj = generate_trajectory(from, to, &params);
    assert_eq!(traj.len(), 10);
}

#[tokio::test]
async fn trajectory_for_long_scroll() {
    let from = Point { x: 200.0, y: 700.0 };
    let to = Point { x: 200.0, y: 50.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
    let durations = generate_durations(traj.len(), 1.2, (0.004, 0.025));
    let sum: f64 = durations.iter().sum();
    assert!((sum - 1.2).abs() < 1e-9);
}
