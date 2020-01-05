pub trait Height1D {
    fn compute(&self, x: f64) -> f64;
}

pub trait Height2D {
    fn compute(&self, x: f64, y: f64) -> f64;

    fn compute_clamped(&self, x: f64, y: f64) -> f64 {
        let val = self.compute(x, y);
        if val < 0.0 {
            0.0
        } else if val > 1.0 {
            1.0
        } else {
            val
        }
    }
}
