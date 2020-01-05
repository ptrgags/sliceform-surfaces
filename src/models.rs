use crate::heights::Height2D;
use crate::surfaces::{
    SineHill, 
};
use crate::height1d::{HeightFunction1D, get_primitive};
use crate::xforms1d::Linear1D;
use crate::csg1d::Union1D;
use crate::csg2d::Complement2D;
use crate::combine::{
    ProductSurface,
    SurfaceOfRevolution, 
    DistanceMetric, 
};
use crate::polynomial::Polynomial;

fn crater_hill() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]).to_box();
    SurfaceOfRevolution::new(poly, DistanceMetric::Euclidean)
}

fn crater_diamond() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]).to_box();
    SurfaceOfRevolution::new(poly, DistanceMetric::Manhattan)
}

fn step_hill() -> SurfaceOfRevolution {
    let step_func = HeightFunction1D::new(steps).to_box();
    SurfaceOfRevolution::new(step_func, DistanceMetric::Euclidean)
}

fn sinc_box() -> SurfaceOfRevolution {
    let sinc_func = HeightFunction1D::new(sinc).to_box();
    SurfaceOfRevolution::new(sinc_func, DistanceMetric::Chessboard)
}

fn peak_rings() -> SurfaceOfRevolution {
    let peak_func = HeightFunction1D::new(double_peak).to_box();
    SurfaceOfRevolution::new(peak_func, DistanceMetric::Chessboard)
}

fn hill(x: f64) -> f64 {
    let y = x - 0.5;
    1.0 - 3.0 * y * y
}

fn peak_thing() -> ProductSurface {
    let peak_func = HeightFunction1D::new(double_peak).to_box();
    let hill_func = HeightFunction1D::new(hill).to_box();

    ProductSurface::new(peak_func, hill_func)
}

fn quad_peak() -> ProductSurface {
    let peak_func = HeightFunction1D::new(double_peak).to_box();
    let peak_func2 = HeightFunction1D::new(double_peak).to_box();
    ProductSurface::new(peak_func, peak_func2)
}

fn nine_peak() -> ProductSurface {
    let peak_func = HeightFunction1D::new(triple_peak).to_box();
    let peak_func2 = HeightFunction1D::new(triple_peak).to_box();
    ProductSurface::new(peak_func, peak_func2)
}

fn nine_tines() -> ProductSurface {
    ProductSurface::new(fork().to_box(), fork().to_box())
}

fn nine_tines_inv() -> Complement2D {
    Complement2D::new(nine_tines().to_box())
}

pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => crater_hill().to_box(),
        "crater_diamond" => crater_diamond().to_box(),
        "step_hill" => step_hill().to_box(),
        "sinc_box" => sinc_box().to_box(),
        "sine_hill" => SineHill::new().to_box(),
        "peak_rings" => peak_rings().to_box(),
        "peak_thing" => peak_thing().to_box(),
        "quad_peak" => quad_peak().to_box(),
        "nine_peak" => nine_peak().to_box(),
        "nine_tines" => nine_tines().to_box(),
        "nine_tines_inv" => nine_tines_inv().to_box(),
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

fn fork() -> Union1D {
    let peak1 = Linear1D::new(get_primitive("peak"), 4.0, 0.0, 0.9, 0.0);
    let peak2 = Linear1D::new(get_primitive("peak"), 4.0, -4.0, 0.9, 0.0);
    let peak3 = Linear1D::new(get_primitive("peak"), 8.0, -4.0, 0.75, 0.0);

    Union1D::many(
        vec![peak1.to_box(), peak2.to_box(), peak3.to_box()])
}
