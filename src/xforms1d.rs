use crate::heights::Height1D;

type Height = Box<dyn Height1D>;

pub struct Linear1D {
    pre_scale: f64,
    pre_shift: f64,
    post_scale: f64,
    post_shift: f64,
    func: Height
}

impl Linear1D {
    pub fn new(
            func: Height, 
            pre_scale: f64, 
            pre_shift: f64, 
            post_scale: f64, 
            post_shift: f64) -> Self {
        Self {
            pre_scale,
            pre_shift,
            post_scale,
            post_shift,
            func
        }
    }

    pub fn identity(func: Height) -> Self {
        Self {
            pre_scale: 1.0,
            pre_shift: 0.0,
            post_scale: 1.0,
            post_shift: 0.0,
            func
        }
    }

    pub fn pre(func: Height, scale: f64, shift: f64) -> Self {
        Self {
            pre_scale: scale,
            pre_shift: shift,
            post_scale: 1.0,
            post_shift: 0.0,
            func
        }
    }
    
    pub fn post(func: Height, scale: f64, shift: f64) -> Self {
        Self {
            pre_scale: 1.0,
            pre_shift: 0.0,
            post_scale: scale,
            post_shift: shift,
            func
        }
    }

    to_box!(Height1D);
}

impl Height1D for Linear1D {
    fn compute(&self, x: f64) -> f64 {
        let pre = self.pre_scale * x + self.pre_shift;
        let val = self.func.compute(pre);
        let post = self.post_scale * val + self.post_shift;

        post
    }
}
