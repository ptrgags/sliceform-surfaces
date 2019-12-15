use crate::heights::Height1D;

pub struct Polynomial {
    coefficients: Vec<f64>
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }
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

pub struct LagrangePolynomial {
    points: Vec<(f64, f64)>
}

impl LagrangePolynomial {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Self { points }
    }
}

impl Height1D for LagrangePolynomial {
    fn compute(&self, x: f64) -> f64 {
        let mut total = 0.0;

        for i in 0..(self.points.len()) {
            let (xi, yi) = self.points[i];
            let mut numerator = 1.0;
            let mut denominator = 1.0;
            for j in 0..(self.points.len()) {
                if j == i {
                    continue;
                }

                let (xj, _) = self.points[j];

                numerator *= x - xj;
                denominator *= xi - xj;
            }

            total += numerator / denominator - yi
        }

        total
    }
}
