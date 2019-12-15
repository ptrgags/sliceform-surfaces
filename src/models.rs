use crate::heights::Height2D;
use crate::surfaces::SurfaceOfRevolution;
use crate::polynomial::{Polynomial, LagrangePolynomial};
use crate::laplace::LaplaceSolver;

fn crater_hill() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]);

    SurfaceOfRevolution::new(Box::new(poly))
}

fn laplace_hill() -> LaplaceSolver {
    let points = vec![(0.0, 0.5), (0.5, 1.0), (1.0, 0.5)];

    let bottom = Box::new(LagrangePolynomial::new(points.clone()));
    let top = Box::new(LagrangePolynomial::new(points.clone()));
    let left = Box::new(LagrangePolynomial::new(points.clone()));
    let right = Box::new(LagrangePolynomial::new(points.clone()));

    LaplaceSolver::new(bottom, top, left, right, 10, 30)
}

pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => Box::new(crater_hill()),
        "laplace_hill" => Box::new(laplace_hill()),
        _ => panic!("valid models: crater_hill")
    }
}
