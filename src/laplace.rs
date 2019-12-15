use std::f64::consts::PI;

use crate::heights::{Height1D, Height2D};

type BoundaryCondition = Box<dyn Height1D>;

/**
 * Solves the Laplace equation laplacian(f) = 0
 * for the box [0, 1] x [0, 1] with four height functions
 * defined at each of the boundary
 */
pub struct LaplaceSolver {
    bottom: BoundaryCondition,
    top: BoundaryCondition,
    left: BoundaryCondition,
    right: BoundaryCondition, 
    terms: u32,
    resolution: u32
}

impl LaplaceSolver {
    pub fn new(
            bottom: BoundaryCondition, 
            top: BoundaryCondition, 
            left: BoundaryCondition, 
            right: BoundaryCondition, 
            terms: u32, 
            resolution: u32) 
            -> Self {
        Self {
            bottom,
            top,
            left,
            right,
            terms,
            resolution
        }
    }
}

impl Height2D for LaplaceSolver {
    fn compute(&self, x: f64, y: f64) -> f64 {
        let mut total = 0.0;

        for n in 0..self.terms {
            let freq = (n as f64) * PI;
            let x_angle = freq * x;
            let y_angle = freq * y;
            let x_wave = x_angle.sin();
            let y_wave = y_angle.sin();
            let x_exp = x_angle.exp() - (2.0 * freq - x_angle).exp();
            let y_exp = y_angle.exp() - (2.0 * freq - y_angle).exp();
            let x_sinh = 2.0 * x_angle.sinh();
            let y_sinh = 2.0 * y_angle.sinh();

            let coeff0 = 2.0 / (1.0 - (2.0 * freq).exp());
            let coeff1 = 2.0 / (freq.exp() - (-freq).exp());

            let bottom_coeff = 
                coeff0 * coeff_integral(&self.bottom, n, self.resolution);
            let top_coeff = 
                coeff1 * coeff_integral(&self.top, n, self.resolution);
            let left_coeff = 
                coeff0 * coeff_integral(&self.left, n, self.resolution);
            let right_coeff = 
                coeff1 * coeff_integral(&self.right, n, self.resolution);

            total +=
                bottom_coeff * x_wave * y_exp + 
                top_coeff * x_wave * y_sinh +
                left_coeff * y_wave * x_exp +
                right_coeff * y_wave * x_sinh;
        }

        total
    }
}

/**
 * Integral from 0 to 1 of f(x)sin(n pi x) dx
 *
 * This uses the LRAM method
 */
fn coeff_integral(f: &BoundaryCondition, n: u32, resolution: u32) -> f64 {
    let mut total = 0.0;

    let dx = 1.0 / (resolution as f64);
    let freq = (n as f64) * PI;

    for i in 0..resolution { 
        let x = (i as f64) * dx;
        let height = f.compute(x);
        let wave = (freq * x).sin();

        total += height * wave * dx;
    }

    total
}
