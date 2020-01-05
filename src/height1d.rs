use crate::heights::Height1D;

type HeightFn1D = fn(f64) -> f64;

pub struct HeightFunction1D {
    height: HeightFn1D
}

impl HeightFunction1D {
    pub fn new(height: HeightFn1D) -> Self {
        Self {
            height
        }
    }

    to_box!(Height1D);
}

impl Height1D for HeightFunction1D {
    fn compute(&self, x: f64) -> f64 {
        (self.height)(x)
    }
}

// Primitive height functions ===========================================

fn peak(x: f64) -> f64 {
    let exponent = -x.abs();
    exponent.exp()
}

fn sin(x: f64) -> f64 {
    x.sin()
}

fn bell(x: f64) -> f64 {
    (-x * x).exp()
}

fn sinc(x: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else {
        x.sin() / x
    }
}

pub fn get_primitive(name: &str) -> Box<dyn Height1D> {
    let func = match name {
        "peak" => peak,
        "sin" => sin,
        "bell" => bell,
        "sinc" => sinc,
        _ => panic!("pick one of: {peak}")
    };

    let wrapper = HeightFunction1D::new(func);
    Box::new(wrapper)
}
