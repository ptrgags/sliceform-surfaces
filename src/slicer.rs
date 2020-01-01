use crate::heights::Height2D;
use crate::geom::Vec2;
use crate::polylines::{Polyline, Slice};

pub struct Slicer {
    slice_res: u32,
    curve_res: u32,
    surface: Box<dyn Height2D>
}

impl Slicer {
    pub fn new(
            slice_res: u32, 
            curve_res: u32, 
            surface: 
            Box<dyn Height2D>) 
            -> Self {
        Self {
            slice_res,
            curve_res,
            surface
        }
    } 

    pub fn make_x_slices(&self) -> Vec<Slice> {
        make_intervals(self.slice_res, false).into_iter().map(|y0| {
            self.make_x_slice(y0) 
        }).collect()
    }

    fn make_x_slice(&self, y0: f64) -> Slice {
        let mut outline_vertices = vec![Vec2(0.0, 0.0), Vec2(1.0, 0.0)]; 
        for x in make_intervals(self.curve_res, true).into_iter().rev() {
            let height = clamp(self.surface.compute(x, y0));
            outline_vertices.push(Vec2(x, height));
        }
        let outline = Polyline::new(&outline_vertices, true);

        let mut slits = Vec::new();
        for x in make_intervals(self.slice_res, false).into_iter() {
            let height = clamp(self.surface.compute(x, y0));
            let slit = Polyline::new(&[
                Vec2(x, height),
                Vec2(x, height / 2.0)
            ], false);

            slits.push(slit);
        }


        Slice::new(outline, slits)
    }

    pub fn make_y_slices(&self) -> Vec<Slice> {
        make_intervals(self.slice_res, false).into_iter().map(|x0| {
            self.make_y_slice(x0) 
        }).collect()
    }

    fn make_y_slice(&self, x0: f64) -> Slice {
        let mut outline_vertices = vec![Vec2(0.0, 0.0), Vec2(1.0, 0.0)]; 
        for y in make_intervals(self.curve_res, true).into_iter() {
            let height = clamp(self.surface.compute(x0, y));
            outline_vertices.push(Vec2(1.0 - y, height));
        }
        let outline = Polyline::new(&outline_vertices, true);

        let mut slits = Vec::new();
        for y in make_intervals(self.slice_res, false).into_iter() {
            let height = clamp(self.surface.compute(x0, y));
            let slit = Polyline::new(&[
                Vec2(1.0 - y, 0.0),
                Vec2(1.0 - y, height / 2.0)
            ], false);

            slits.push(slit);
        }

        Slice::new(outline, slits)
    }
}


fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn make_intervals(max_depth: u32, include_endpoints: bool) -> Vec<f64> {
    let mut result = vec![];
    
    if include_endpoints {
        result.push(0.0);
    }

    result.append(&mut intervals(0.0, 1.0, 0, max_depth));

    if include_endpoints {
        result.push(1.0);
    }

    result
}

fn intervals(left: f64, right: f64, depth: u32, max_depth: u32) -> Vec<f64> {
    let mid = (left + right) / 2.0;
    if depth == max_depth {
        return vec![mid];
    }

    let mut result = vec![];
    result.append(&mut intervals(left, mid, depth + 1, max_depth));
    result.push(mid);
    result.append(&mut intervals(mid, right, depth + 1, max_depth));

    result
}
