pub trait Height2D {
    fn compute(&self, x: f64, y: f64) -> f64;
}

pub trait Height1D {
    fn compute(&self, x: f64) -> f64;
}
