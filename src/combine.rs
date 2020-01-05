use crate::heights::{Height1D, Height2D};

// Product of two surfaces =================================================

pub struct ProductSurface {
    x_func: Box<dyn Height1D>,
    y_func: Box<dyn Height1D>,
}

impl ProductSurface {
    pub fn new(x_func: Box<dyn Height1D>, y_func: Box<dyn Height1D>) -> Self {
        Self {
            x_func,
            y_func,
        }
    }

    to_box!(Height2D);
}

impl Height2D for ProductSurface {
    fn compute(&self, x: f64, y:f64) -> f64 {
        let x = self.x_func.compute(x);
        let y = self.y_func.compute(y);

        x * y
    }
}

// Create surfaces of revolution ===========================================

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

    to_box!(Height2D);
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
