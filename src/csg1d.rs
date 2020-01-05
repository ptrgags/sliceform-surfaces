use std::f64::{INFINITY, NEG_INFINITY};

use crate::heights::Height1D;

type Height = Box<dyn Height1D>;

pub struct Union1D {
    funcs: Vec<Height>
}

impl Union1D {
    pub fn new(a: Height, b: Height) -> Self {
        Self {
            funcs: vec![a, b]
        }
    }

    pub fn many(funcs: Vec<Height>) -> Self {
        Self {
            funcs
        }
    }

    to_box!(Height1D);
}

impl Height1D for Union1D {
    fn compute(&self, x: f64) -> f64 {
        let mut result = NEG_INFINITY;
        for func in self.funcs.iter() {
            let val = func.compute(x);
            result = result.max(val);
        }

        result
    }
}

pub struct Intersection1D {
    funcs: Vec<Height>
}

impl Intersection1D {
    pub fn new(a: Height, b: Height) -> Self {
        Self {
            funcs: vec![a, b]
        }
    }

    pub fn many(funcs: Vec<Height>) -> Self {
        Self {
            funcs
        }
    }

    to_box!(Height1D);
}

impl Height1D for Intersection1D {
    fn compute(&self, x: f64) -> f64 {
        let mut result = INFINITY;
        for func in self.funcs.iter() {
            let val = func.compute(x);
            result = result.min(val);
        }

        result
    }
}
