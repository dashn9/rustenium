use rand::Rng;
use super::mouse::Point;

// ── Easing functions ─────────────────────────────────────────────────────────

fn ease_out_cubic(t: f64) -> f64 { let t = t - 1.0; t * t * t + 1.0 }
fn ease_out_quart(t: f64) -> f64 { let t = t - 1.0; -(t * t * t * t - 1.0) }
fn ease_out_quint(t: f64) -> f64 { let t = t - 1.0; t * t * t * t * t + 1.0 }
fn ease_out_expo(t: f64) -> f64 { if t == 1.0 { 1.0 } else { -f64::powf(2.0, -10.0 * t) + 1.0 } }
fn ease_out_sine(t: f64) -> f64 { (t * std::f64::consts::FRAC_PI_2).sin() }
fn ease_out_circ(t: f64) -> f64 { let t = t - 1.0; (1.0 - t * t).sqrt() }
fn ease_in_out_cubic(t: f64) -> f64 { if t < 0.5 { 4.0 * t * t * t } else { let t = 2.0 * t - 2.0; 0.5 * t * t * t + 1.0 } }
fn ease_in_out_quart(t: f64) -> f64 { if t < 0.5 { 8.0 * t * t * t * t } else { let t = t - 1.0; -8.0 * t * t * t * t + 1.0 } }
fn ease_in_out_quint(t: f64) -> f64 { if t < 0.5 { 16.0 * t * t * t * t * t } else { let t = 2.0 * t - 2.0; 0.5 * t * t * t * t * t + 1.0 } }
fn ease_in_out_sine(t: f64) -> f64 { -0.5 * ((std::f64::consts::PI * t).cos() - 1.0) }
fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 { return 0.0; }
    if t == 1.0 { return 1.0; }
    if t < 0.5 { 0.5 * f64::powf(2.0, 20.0 * t - 10.0) } else { 1.0 - 0.5 * f64::powf(2.0, -20.0 * t + 10.0) }
}
fn ease_in_out_circ(t: f64) -> f64 {
    if t < 0.5 { 0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt()) } else { 0.5 * ((1.0 - (2.0 * t - 2.0).powi(2)).sqrt() + 1.0) }
}
pub(crate) fn linear(t: f64) -> f64 { t }

pub(crate) type EasingFn = fn(f64) -> f64;

pub(crate) const EASING_FUNCTIONS: &[EasingFn] = &[
    ease_out_expo, ease_in_out_quint, ease_in_out_sine, ease_in_out_quart,
    ease_in_out_expo, ease_in_out_cubic, ease_in_out_circ, linear,
    ease_out_sine, ease_out_quart, ease_out_quint, ease_out_cubic, ease_out_circ,
];

// ── Bezier (N-degree) ────────────────────────────────────────────────────────

fn binomial(n: u64, k: u64) -> f64 {
    if k > n { return 0.0; }
    let mut result = 1.0;
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result
}

fn bernstein(t: f64, i: u64, n: u64) -> f64 {
    binomial(n, i) * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

fn bezier_point(t: f64, control_points: &[Point]) -> Point {
    let n = (control_points.len() - 1) as u64;
    let (mut x, mut y) = (0.0, 0.0);
    for (i, cp) in control_points.iter().enumerate() {
        let b = bernstein(t, i as u64, n);
        x += cp.x * b;
        y += cp.y * b;
    }
    Point { x, y }
}

fn bezier_curve(num_points: usize, control_points: &[Point]) -> Vec<Point> {
    (0..num_points)
        .map(|i| bezier_point(i as f64 / (num_points - 1).max(1) as f64, control_points))
        .collect()
}

// ── Curve parameters & generation ────────────────────────────────────────────

pub struct CurveParams {
    pub offset_boundary_x: f64,
    pub offset_boundary_y: f64,
    pub knots_count: usize,
    pub distortion_mean: f64,
    pub distortion_stdev: f64,
    pub distortion_frequency: f64,
    pub tween: EasingFn,
    pub target_points: usize,
}

pub fn random_curve_params(_from: Point, _to: Point) -> CurveParams {
    let mut rng = rand::rng();

    let tween = EASING_FUNCTIONS[rng.random_range(0..EASING_FUNCTIONS.len())];
    let offset_boundary_x = weighted_range_pick(&mut rng, &[(20, 45, 0.2), (45, 75, 0.65), (75, 100, 0.15)]);
    let offset_boundary_y = weighted_range_pick(&mut rng, &[(20, 45, 0.2), (45, 75, 0.65), (75, 100, 0.15)]);

    let knots_count = weighted_pick(&mut rng, &[
        (1, 0.15), (2, 0.36), (3, 0.17), (4, 0.12), (5, 0.08),
        (6, 0.04), (7, 0.03), (8, 0.02), (9, 0.015), (10, 0.005),
    ]);

    let distortion_mean = rng.random_range(80..=110) as f64 / 100.0;
    let distortion_stdev = rng.random_range(85..=110) as f64 / 100.0;
    let distortion_frequency = rng.random_range(25..=70) as f64 / 100.0;
    let target_points = weighted_range_pick(&mut rng, &[(35, 45, 0.53), (45, 60, 0.32), (60, 80, 0.15)]) as usize;

    CurveParams {
        offset_boundary_x, offset_boundary_y, knots_count,
        distortion_mean, distortion_stdev, distortion_frequency,
        tween, target_points,
    }
}

pub fn generate_trajectory(from: Point, to: Point, params: &CurveParams) -> Vec<Point> {
    let mut rng = rand::rng();

    let left = from.x.min(to.x) - params.offset_boundary_x;
    let right = from.x.max(to.x) + params.offset_boundary_x;
    let bottom = from.y.min(to.y) - params.offset_boundary_y;
    let top = from.y.max(to.y) + params.offset_boundary_y;

    let mut control_points = vec![from];
    for _ in 0..params.knots_count {
        control_points.push(Point {
            x: rng.random_range(left..=right),
            y: rng.random_range(bottom..=top),
        });
    }
    control_points.push(to);

    let mid_points = ((from.x - to.x).abs().max((from.y - to.y).abs()) as usize).max(2);
    let mut points = bezier_curve(mid_points, &control_points);

    for i in 1..points.len().saturating_sub(1) {
        if rng.random_range(0.0..1.0) < params.distortion_frequency {
            let delta = gauss(&mut rng, params.distortion_mean, params.distortion_stdev);
            points[i].y += delta;
        }
    }

    let tween = params.tween;
    let n = params.target_points.max(2);
    (0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            let index = (tween(t) * (points.len() - 1) as f64) as usize;
            points[index.min(points.len() - 1)]
        })
        .collect()
}

// ── Utilities ────────────────────────────────────────────────────────────────

fn weighted_range_pick(rng: &mut impl Rng, ranges: &[(i32, i32, f64)]) -> f64 {
    let total: f64 = ranges.iter().map(|r| r.2).sum();
    let mut roll = rng.random_range(0.0..total);
    for &(lo, hi, weight) in ranges {
        roll -= weight;
        if roll <= 0.0 {
            return rng.random_range(lo..hi) as f64;
        }
    }
    ranges.last().map(|r| rng.random_range(r.0..r.1) as f64).unwrap_or(50.0)
}

pub(crate) fn weighted_pick(rng: &mut impl Rng, items: &[(usize, f64)]) -> usize {
    let total: f64 = items.iter().map(|i| i.1).sum();
    let mut roll = rng.random_range(0.0..total);
    for &(val, weight) in items {
        roll -= weight;
        if roll <= 0.0 { return val; }
    }
    items.last().map(|i| i.0).unwrap_or(2)
}

pub(crate) fn gauss(rng: &mut impl Rng, mean: f64, stdev: f64) -> f64 {
    let u1: f64 = rng.random_range(0.0001..1.0);
    let u2: f64 = rng.random_range(0.0..std::f64::consts::TAU);
    mean + stdev * (-2.0 * u1.ln()).sqrt() * u2.cos()
}

/// Generate `count` random durations in `latency_range` that sum to `total_secs`.
pub fn generate_durations(count: usize, total_secs: f64, latency_range: (f64, f64)) -> Vec<f64> {
    if count == 0 { return vec![]; }
    let mut rng = rand::rng();
    let mut durations: Vec<f64> = (0..count)
        .map(|_| rng.random_range(latency_range.0..=latency_range.1))
        .collect();
    let sum: f64 = durations.iter().sum();
    if sum > 0.0 {
        let scale = total_secs / sum;
        for d in &mut durations { *d *= scale; }
    }
    durations
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::mouse::Point;

    #[test]
    fn bezier_point_at_t0_returns_start() {
        let cps = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 50.0, y: 100.0 },
            Point { x: 100.0, y: 0.0 },
        ];
        let p = bezier_point(0.0, &cps);
        assert!((p.x).abs() < f64::EPSILON);
        assert!((p.y).abs() < f64::EPSILON);
    }

    #[test]
    fn bezier_point_at_t1_returns_end() {
        let cps = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 50.0, y: 100.0 },
            Point { x: 200.0, y: 50.0 },
        ];
        let p = bezier_point(1.0, &cps);
        assert!((p.x - 200.0).abs() < 1e-9);
        assert!((p.y - 50.0).abs() < 1e-9);
    }

    #[test]
    fn bezier_curve_produces_correct_count() {
        let cps = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 50.0, y: 100.0 },
            Point { x: 100.0, y: 0.0 },
        ];
        let pts = bezier_curve(50, &cps);
        assert_eq!(pts.len(), 50);
    }

    #[test]
    fn bezier_curve_first_and_last_match_endpoints() {
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

    #[test]
    fn n_degree_bezier_with_many_control_points() {
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

    #[test]
    fn binomial_coefficients() {
        assert!((binomial(4, 2) - 6.0).abs() < 1e-9);
        assert!((binomial(5, 0) - 1.0).abs() < 1e-9);
        assert!((binomial(5, 5) - 1.0).abs() < 1e-9);
        assert!((binomial(10, 3) - 120.0).abs() < 1e-9);
    }

    #[test]
    fn binomial_edge_cases() {
        assert!((binomial(0, 0) - 1.0).abs() < 1e-9);
        assert!((binomial(1, 0) - 1.0).abs() < 1e-9);
        assert!((binomial(1, 1) - 1.0).abs() < 1e-9);
        assert!((binomial(3, 4)).abs() < 1e-9);
    }

    #[test]
    fn easing_functions_boundaries() {
        for f in EASING_FUNCTIONS {
            let at_0 = f(0.0);
            let at_1 = f(1.0);
            assert!((at_0).abs() < 1e-9, "easing(0) should be ~0, got {}", at_0);
            assert!((at_1 - 1.0).abs() < 1e-9, "easing(1) should be ~1, got {}", at_1);
        }
    }

    #[test]
    fn linear_easing_is_identity() {
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            assert!((linear(t) - t).abs() < 1e-9);
        }
    }

    #[test]
    fn gauss_distribution_reasonable_mean() {
        let mut rng = rand::rng();
        let mut sum = 0.0;
        let n = 1000;
        for _ in 0..n {
            sum += gauss(&mut rng, 1.0, 1.0);
        }
        let mean = sum / n as f64;
        assert!((mean - 1.0).abs() < 0.2, "gauss mean should be ~1.0, got {}", mean);
    }

    #[test]
    fn gauss_with_zero_stdev_returns_mean() {
        let mut rng = rand::rng();
        for _ in 0..50 {
            let v = gauss(&mut rng, 5.0, 0.0);
            assert!((v - 5.0).abs() < 1e-9);
        }
    }

    #[test]
    fn weighted_pick_returns_valid_values() {
        let mut rng = rand::rng();
        let items: &[(usize, f64)] = &[(1, 0.5), (2, 0.3), (3, 0.2)];
        for _ in 0..100 {
            let v = weighted_pick(&mut rng, items);
            assert!(v >= 1 && v <= 3);
        }
    }

    #[test]
    fn weighted_pick_respects_weights() {
        let mut rng = rand::rng();
        let items: &[(usize, f64)] = &[(1, 0.99), (2, 0.01)];
        let mut count_1 = 0;
        for _ in 0..1000 {
            if weighted_pick(&mut rng, items) == 1 { count_1 += 1; }
        }
        assert!(count_1 > 900, "heavily weighted item should appear >90%, got {}", count_1);
    }
}
