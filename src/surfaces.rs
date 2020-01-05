use std::f64::consts::PI;

use crate::heights::Height2D;

pub struct SineHill {}

impl SineHill {
    pub fn new() -> Self {
        Self {}
    }

    to_box!(Height2D);
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
