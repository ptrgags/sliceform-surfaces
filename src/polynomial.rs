use crate::heights::Height1D;

pub struct Polynomial {
    coefficients: Vec<f64>
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }

    to_box!(Height1D);
}

impl Height1D for Polynomial {
    fn compute(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut x_power = 1.0; // x^0 = 1
        for coeff in self.coefficients.iter() {
            result += *coeff * x_power; 
            x_power *= x;
        }

        result
    }
}
