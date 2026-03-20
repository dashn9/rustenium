use rustenium::input::{
    Point, bezier_point, bezier_curve, binomial, generate_trajectory,
    random_curve_params, gauss, weighted_pick, EASING_FUNCTIONS, linear, CurveParams,
};

#[tokio::test]
async fn bezier_point_at_t0_returns_start() {
    let cps = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 50.0, y: 100.0 },
        Point { x: 100.0, y: 0.0 },
    ];
    let p = bezier_point(0.0, &cps);
    assert!((p.x).abs() < f64::EPSILON);
    assert!((p.y).abs() < f64::EPSILON);
}

#[tokio::test]
async fn bezier_point_at_t1_returns_end() {
    let cps = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 50.0, y: 100.0 },
        Point { x: 200.0, y: 50.0 },
    ];
    let p = bezier_point(1.0, &cps);
    assert!((p.x - 200.0).abs() < 1e-9);
    assert!((p.y - 50.0).abs() < 1e-9);
}

#[tokio::test]
async fn bezier_curve_produces_correct_count() {
    let cps = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 50.0, y: 100.0 },
        Point { x: 100.0, y: 0.0 },
    ];
    let pts = bezier_curve(50, &cps);
    assert_eq!(pts.len(), 50);
}

#[tokio::test]
async fn bezier_curve_first_and_last_match_endpoints() {
    let cps = vec![
        Point { x: 10.0, y: 20.0 },
        Point { x: 50.0, y: 80.0 },
        Point { x: 90.0, y: 20.0 },
    ];
    let pts = bezier_curve(100, &cps);
    assert!((pts[0].x - 10.0).abs() < 1e-9);
    assert!((pts[0].y - 20.0).abs() < 1e-9);
    assert!((pts[99].x - 90.0).abs() < 1e-9);
    assert!((pts[99].y - 20.0).abs() < 1e-9);
}

#[tokio::test]
async fn n_degree_bezier_with_many_control_points() {
    let cps = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 20.0, y: 80.0 },
        Point { x: 40.0, y: 10.0 },
        Point { x: 60.0, y: 90.0 },
        Point { x: 80.0, y: 20.0 },
        Point { x: 100.0, y: 100.0 },
    ];
    let start = bezier_point(0.0, &cps);
    let end = bezier_point(1.0, &cps);
    assert!((start.x).abs() < 1e-9);
    assert!((end.x - 100.0).abs() < 1e-9);
    assert!((end.y - 100.0).abs() < 1e-9);
}

#[tokio::test]
async fn binomial_coefficients() {
    assert!((binomial(4, 2) - 6.0).abs() < 1e-9);
    assert!((binomial(5, 0) - 1.0).abs() < 1e-9);
    assert!((binomial(5, 5) - 1.0).abs() < 1e-9);
    assert!((binomial(10, 3) - 120.0).abs() < 1e-9);
}

#[tokio::test]
async fn binomial_edge_cases() {
    assert!((binomial(0, 0) - 1.0).abs() < 1e-9);
    assert!((binomial(1, 0) - 1.0).abs() < 1e-9);
    assert!((binomial(1, 1) - 1.0).abs() < 1e-9);
    assert!((binomial(3, 4)).abs() < 1e-9); // k > n
}

#[tokio::test]
async fn easing_functions_boundaries() {
    for f in EASING_FUNCTIONS {
        let at_0 = f(0.0);
        let at_1 = f(1.0);
        assert!((at_0).abs() < 1e-9, "easing(0) should be ~0, got {}", at_0);
        assert!((at_1 - 1.0).abs() < 1e-9, "easing(1) should be ~1, got {}", at_1);
    }
}

#[tokio::test]
async fn easing_functions_monotonic_at_boundaries() {
    for f in EASING_FUNCTIONS {
        let near_0 = f(0.01);
        let near_1 = f(0.99);
        assert!(near_0 >= 0.0, "easing(0.01) should be >= 0");
        assert!(near_1 <= 1.0 + 1e-9, "easing(0.99) should be <= 1");
    }
}

#[tokio::test]
async fn linear_easing_is_identity() {
    for i in 0..=100 {
        let t = i as f64 / 100.0;
        assert!((linear(t) - t).abs() < 1e-9);
    }
}

#[tokio::test]
async fn gauss_distribution_reasonable_mean() {
    let mut rng = rand::rng();
    let mut sum = 0.0;
    let n = 1000;
    for _ in 0..n {
        sum += gauss(&mut rng, 1.0, 1.0);
    }
    let mean = sum / n as f64;
    assert!((mean - 1.0).abs() < 0.2, "gauss mean should be ~1.0, got {}", mean);
}

#[tokio::test]
async fn gauss_with_zero_stdev_returns_mean() {
    let mut rng = rand::rng();
    for _ in 0..50 {
        let v = gauss(&mut rng, 5.0, 0.0);
        assert!((v - 5.0).abs() < 1e-9);
    }
}

#[tokio::test]
async fn weighted_pick_returns_valid_values() {
    let mut rng = rand::rng();
    let items: &[(usize, f64)] = &[(1, 0.5), (2, 0.3), (3, 0.2)];
    for _ in 0..100 {
        let v = weighted_pick(&mut rng, items);
        assert!(v >= 1 && v <= 3);
    }
}

#[tokio::test]
async fn weighted_pick_respects_weights() {
    let mut rng = rand::rng();
    let items: &[(usize, f64)] = &[(1, 0.99), (2, 0.01)];
    let mut count_1 = 0;
    for _ in 0..1000 {
        if weighted_pick(&mut rng, items) == 1 { count_1 += 1; }
    }
    assert!(count_1 > 900, "heavily weighted item should appear >90%, got {}", count_1);
}

#[tokio::test]
async fn random_curve_params_ranges() {
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

#[tokio::test]
async fn generate_trajectory_produces_points() {
    let from = Point { x: 10.0, y: 20.0 };
    let to = Point { x: 300.0, y: 400.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2, "trajectory should have at least 2 points");
}

#[tokio::test]
async fn generate_trajectory_endpoints_close() {
    let from = Point { x: 50.0, y: 50.0 };
    let to = Point { x: 400.0, y: 300.0 };
    let params = CurveParams {
        offset_boundary_x: 30.0,
        offset_boundary_y: 30.0,
        knots_count: 2,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: linear,
        target_points: 50,
    };
    let traj = generate_trajectory(from, to, &params);
    let first = traj.first().unwrap();
    let last = traj.last().unwrap();
    assert!((first.x - from.x).abs() < 5.0, "first x off: {}", (first.x - from.x).abs());
    assert!((first.y - from.y).abs() < 5.0, "first y off: {}", (first.y - from.y).abs());
    assert!((last.x - to.x).abs() < 5.0, "last x off: {}", (last.x - to.x).abs());
    assert!((last.y - to.y).abs() < 5.0, "last y off: {}", (last.y - to.y).abs());
}

#[tokio::test]
async fn generate_trajectory_no_distortion_smooth() {
    let from = Point { x: 0.0, y: 0.0 };
    let to = Point { x: 200.0, y: 0.0 };
    let params = CurveParams {
        offset_boundary_x: 0.1,
        offset_boundary_y: 0.1,
        knots_count: 0,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: linear,
        target_points: 50,
    };
    let traj = generate_trajectory(from, to, &params);
    // With no knots and no distortion, should be roughly a straight line
    for pt in &traj {
        assert!(pt.y.abs() < 2.0, "y should be near 0 for straight line, got {}", pt.y);
    }
}

#[tokio::test]
async fn generate_trajectory_with_many_knots_still_valid() {
    let from = Point { x: 0.0, y: 0.0 };
    let to = Point { x: 100.0, y: 100.0 };
    let params = CurveParams {
        offset_boundary_x: 50.0,
        offset_boundary_y: 50.0,
        knots_count: 10,
        distortion_mean: 1.0,
        distortion_stdev: 1.0,
        distortion_frequency: 0.5,
        tween: linear,
        target_points: 60,
    };
    let traj = generate_trajectory(from, to, &params);
    assert_eq!(traj.len(), 60);
}

#[tokio::test]
async fn generate_trajectory_short_distance() {
    let from = Point { x: 100.0, y: 100.0 };
    let to = Point { x: 102.0, y: 101.0 };
    let params = random_curve_params(from, to);
    let traj = generate_trajectory(from, to, &params);
    assert!(traj.len() >= 2);
}

#[tokio::test]
async fn generate_trajectory_same_point() {
    let p = Point { x: 50.0, y: 50.0 };
    let params = CurveParams {
        offset_boundary_x: 10.0,
        offset_boundary_y: 10.0,
        knots_count: 1,
        distortion_mean: 0.0,
        distortion_stdev: 0.0,
        distortion_frequency: 0.0,
        tween: linear,
        target_points: 10,
    };
    let traj = generate_trajectory(p, p, &params);
    assert!(traj.len() >= 2);
}
