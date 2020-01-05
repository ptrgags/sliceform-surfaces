use crate::heights::Height2D;

type Height = Box<dyn Height2D>;

pub struct Complement2D {
    func: Height
}

impl Complement2D {
    pub fn new(func: Height) -> Self {
        Self {
            func
        }
    }

    to_box!(Height2D);
}

impl Height2D for Complement2D {
    fn compute(&self, x: f64, y: f64) -> f64 {
        1.0 - self.func.compute(x, y)
    }
}
