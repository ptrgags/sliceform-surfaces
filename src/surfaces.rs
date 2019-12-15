use crate::heights::{Height1D, Height2D};

pub struct SurfaceOfRevolution {
    cross_section: Box<dyn Height1D> 
}

impl SurfaceOfRevolution {
    pub fn new(cross_section: Box<dyn Height1D>) -> Self {
        Self { cross_section }
    }
}

impl Height2D for SurfaceOfRevolution {
    fn compute(&self, x: f64, y:f64) -> f64 {
        let (x2, y2) = to_centered(x, y);
        let (r, _) = to_polar(x2, y2);

        self.cross_section.compute(r)  
    }
}

fn to_centered(x: f64, y: f64) -> (f64, f64) {
    (2.0 * x - 1.0, 2.0 * y - 1.0)
}

fn to_polar(x: f64, y: f64) -> (f64, f64) {
    let r = (x * x + y * y).sqrt();
    let theta = y.atan2(x);

    (r, theta)
}
