use std::f64::consts::PI;

use crate::heights::{Height1D, Height2D};

pub enum DistanceMetric {
    Manhattan,
    Euclidean,
    Chessboard,
}

pub struct SurfaceOfRevolution {
    cross_section: Box<dyn Height1D>,
    metric: DistanceMetric,
}

impl SurfaceOfRevolution {
    pub fn new(cross_section: Box<dyn Height1D>, metric: DistanceMetric) -> Self {
        Self { cross_section, metric }
    }
}

impl Height2D for SurfaceOfRevolution {
    fn compute(&self, x: f64, y:f64) -> f64 {
        let (x2, y2) = to_centered(x, y);
        let (r, _) = to_polar(x2, y2, &self.metric);

        self.cross_section.compute(r)  
    }
}

fn to_centered(x: f64, y: f64) -> (f64, f64) {
    (2.0 * x - 1.0, 2.0 * y - 1.0)
}

fn to_polar(x: f64, y: f64, metric: &DistanceMetric) -> (f64, f64) {
    use DistanceMetric::*;
    let r = match metric {
        Manhattan => x.abs() + y.abs(),
        Euclidean => (x * x + y * y).sqrt(),
        Chessboard => x.abs().max(y.abs()),
    };
    let theta = y.atan2(x);

    (r, theta)
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

type HeightFn = fn(f64) -> f64;

pub struct HeightFunction {
    height: HeightFn
}

impl HeightFunction {
    pub fn new(height: HeightFn) -> Self {
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

pub struct ProductSurface {
    x_func: HeightFn,
    y_func: HeightFn,
}

impl ProductSurface {
    pub fn new(x_func: HeightFn, y_func: HeightFn) -> Self {
        Self {
            x_func,
            y_func,
        }
    }
}

impl Height2D for ProductSurface {
    fn compute(&self, x: f64, y:f64) -> f64 {
        let x = (self.x_func)(x);
        let y = (self.y_func)(y);

        x * y
    }
}
