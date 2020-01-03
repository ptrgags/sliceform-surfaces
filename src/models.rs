use crate::heights::Height2D;
use crate::surfaces::{
    SurfaceOfRevolution, 
    DistanceMetric, 
    SineHill, 
    HeightFunction,
    ProductSurface,
};
use crate::polynomial::Polynomial;

fn crater_hill() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]);
    SurfaceOfRevolution::new(Box::new(poly), DistanceMetric::Euclidean)
}

fn crater_diamond() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]);
    SurfaceOfRevolution::new(Box::new(poly), DistanceMetric::Manhattan)
}

fn step_hill() -> SurfaceOfRevolution {
    let step_func = HeightFunction::new(steps);
    SurfaceOfRevolution::new(Box::new(step_func), DistanceMetric::Euclidean)
}

fn sinc_box() -> SurfaceOfRevolution {
    let sinc_func = HeightFunction::new(sinc);
    SurfaceOfRevolution::new(Box::new(sinc_func), DistanceMetric::Chessboard)
}

fn peak_rings() -> SurfaceOfRevolution {
    let peak_func = HeightFunction::new(double_peak);
    SurfaceOfRevolution::new(Box::new(peak_func), DistanceMetric::Chessboard)
}

fn hill(x: f64) -> f64 {
    let y = x - 0.5;
    1.0 - 3.0 * y * y
}

fn peak_thing() -> ProductSurface {
    let peak_func = double_peak;
    let hill_func = hill;

    ProductSurface::new(peak_func, hill_func)
}

fn quad_peak() -> ProductSurface {
    ProductSurface::new(double_peak, double_peak)
}

fn nine_peak() -> ProductSurface {
    ProductSurface::new(triple_peak, triple_peak)
}

pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => Box::new(crater_hill()),
        "crater_diamond" => Box::new(crater_diamond()),
        "step_hill" => Box::new(step_hill()),
        "sinc_box" => Box::new(sinc_box()),
        "sine_hill" => Box::new(SineHill::new()),
        "peak_rings" => Box::new(peak_rings()),
        "peak_thing" => Box::new(peak_thing()),
        "quad_peak" => Box::new(quad_peak()),
        "nine_peak" => Box::new(nine_peak()),
        _ => panic!("valid models: crater_hill, step_hill, sinc_box")
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn scaled_t(t: f64, t0: f64, t1: f64) -> f64 {
    (t - t0) / (t1 - t0)
}

fn lerp_heights(a: f64, b: f64, t: f64, t0: f64, t1: f64) -> f64 {
    lerp(a, b, scaled_t(t, t0, t1))
}

fn sinc(r: f64) -> f64 {
    if r == 0.0 {
        5.0 / 8.0 + 0.35
    } else {
        5.0 / 8.0 * (10.0 * r).sin() + 0.35
    }
}

fn steps(r: f64) -> f64 {
    if r <= 0.2 {
        1.0
    } else if r <= 0.3 {
        lerp_heights(1.0, 0.9, r, 0.2, 0.3)
    } else if r <= 0.4 {
        0.9
    } else if r <= 0.5 {
        lerp_heights(0.9, 0.7, r, 0.4, 0.5)
    } else if r <= 0.6 {
        0.7
    } else if r <= 0.7 {
        lerp_heights(0.7, 0.4, r, 0.6, 0.7)
    } else if r <= 0.9 {
        0.4
    } else if r <= 1.0 {
        lerp_heights(0.4, 0.2, r, 0.9, 1.0)
    } else {
        0.2
    }
}

fn peak(x: f64, n: f64) -> f64 {
    let exponent = -n * x.abs();
    exponent.exp()
}

fn double_peak(x: f64) -> f64 {
    const A: f64 = 0.25;
    const B: f64 = 0.75;
    const C: f64 = 0.9;
    const D: f64 = 0.7;
    const N: f64 = 3.8;

    C * peak(x - A, N) + D * peak(x - B, N)
}

fn triple_peak(x: f64) -> f64 {
    const A: f64 = 0.25;
    const B: f64 = 0.5; 
    const C: f64 = 0.75;

    const D: f64 = 0.4;
    const E: f64 = 0.9;
    const N: f64 = 10.0;
    1.2 * (D * peak(x - A, N) + E * peak(x - B, N) + D * peak(x - C, N))
}
