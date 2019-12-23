use std::f64::consts::PI;

use crate::heights::{Height1D, Height2D};
use crate::surfaces::SurfaceOfRevolution;
use crate::polynomial::Polynomial;
use crate::surfaces::DistanceMetric;

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


pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => Box::new(crater_hill()),
        "crater_diamond" => Box::new(crater_diamond()),
        "step_hill" => Box::new(step_hill()),
        "sinc_box" => Box::new(sinc_box()),
        "sine_hill" => Box::new(SineHill::new()),
        _ => panic!("valid models: crater_hill, step_hill, sinc_box")
    }
}

pub struct SineHill {}

impl SineHill {
    pub fn new() -> Self {
        Self {}
    }
}

impl Height2D for SineHill {
    fn compute(&self, x: f64, y: f64) -> f64 {
        const FREQ: f64 = 1.58;
        const PHASE: f64 = 2.0;
        let sine = 0.5 + 0.25 * (2.0 * PI * FREQ * x - PHASE).sin();

        const SHARPNESS:f64 = 6.0;
        let shifted = y - 0.5;
        let hill = (-SHARPNESS * shifted * shifted).exp();

        sine * hill
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
