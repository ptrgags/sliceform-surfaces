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
