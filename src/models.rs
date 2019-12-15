use crate::heights::Height2D;
use crate::surfaces::SurfaceOfRevolution;
use crate::polynomial::Polynomial;

fn crater_hill() -> SurfaceOfRevolution {
    let poly = Polynomial::new(vec![0.5, -1.4, 6.7, -5.5]);

    SurfaceOfRevolution::new(Box::new(poly))
}

pub fn select_model(name: &str) -> Box<dyn Height2D> {
    match name {
        "crater_hill" => Box::new(crater_hill()),
        _ => panic!("valid models: crater_hill")
    }
}
