use crate::heights::{Height1D, Height2D};
use crate::surfaces::SurfaceOfRevolution;
use crate::polynomial::Polynomial;

fn crater_hill() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]);

    SurfaceOfRevolution::new(Box::new(poly))
}

fn step_hill() -> SurfaceOfRevolution {
    let step_func = HeightFunction::new(steps);
    
    SurfaceOfRevolution::new(Box::new(step_func))
}

pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => Box::new(crater_hill()),
        "step_hill" => Box::new(step_hill()),
        _ => panic!("valid models: crater_hill")
    }
}

pub struct HeightFunction {
    height: fn(f64) -> f64
}

impl HeightFunction {
    pub fn new(height: fn(f64) -> f64) -> Self {
        Self {
            height
        }
    }
}

impl Height1D for HeightFunction {
    fn compute(&self, x: f64) -> f64 {
        (self.height)(x)
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
