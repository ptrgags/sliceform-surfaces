mod polylines;
mod printer;

use polylines::{Vec2, Polyline, Slice};
use printer::Printer;

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

type Surface = fn(f64, f64) -> f64;

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn x_slices(surf: Surface, slice_resolution: u32, curve_resolution: u32) 
        -> Vec<Slice> {
    make_intervals(slice_resolution, false).into_iter().map(|y0| {
        x_slice(surf, y0, slice_resolution, curve_resolution) 
    }).collect()
}

fn x_slice(
        surf: Surface, y0: f64, slice_resolution: u32, curve_resolution: u32) 
        -> Slice {
    let mut outline_vertices = vec![Vec2(0.0, 0.0), Vec2(1.0, 0.0)]; 
    for x in make_intervals(curve_resolution, true).into_iter().rev() {
        let height = clamp(surf(x, y0));
        outline_vertices.push(Vec2(x, height));
    }
    let outline = Polyline::new(&outline_vertices, true);

    let mut slits = Vec::new();
    for x in make_intervals(slice_resolution, false).into_iter() {
        let height = clamp(surf(x, y0));
        let slit = Polyline::new(&[
            Vec2(x, height),
            Vec2(x, height / 2.0)
        ], false);

        slits.push(slit);
    }


    Slice::new(outline, slits)
}

fn y_slices(surf: Surface, slice_resolution: u32, curve_resolution: u32) 
        -> Vec<Slice> {
    make_intervals(slice_resolution, false).into_iter().map(|x0| {
        y_slice(surf, x0, slice_resolution, curve_resolution) 
    }).collect()
}

fn y_slice(
        surf: Surface, x0: f64, slice_resolution: u32, curve_resolution: u32) 
        -> Slice {
    let mut outline_vertices = vec![Vec2(0.0, 0.0), Vec2(1.0, 0.0)]; 
    for y in make_intervals(curve_resolution, true).into_iter() {
        let height = clamp(surf(x0, y));
        outline_vertices.push(Vec2(1.0 - y, height));
    }
    let outline = Polyline::new(&outline_vertices, true);

    let mut slits = Vec::new();
    for y in make_intervals(slice_resolution, false).into_iter() {
        let height = clamp(surf(x0, y));
        let slit = Polyline::new(&[
            Vec2(1.0 - y, 0.0),
            Vec2(1.0 - y, height / 2.0)
        ], false);

        slits.push(slit);
    }

    Slice::new(outline, slits)
}

fn to_centered(x: f64, y: f64) -> (f64, f64) {
    (2.0 * x - 1.0, 2.0 * y - 1.0)
}

fn to_polar(x: f64, y: f64) -> (f64, f64) {
    let r = (x * x + y * y).sqrt();
    let theta = y.atan2(x);

    (r, theta)
}

fn product(x: f64, y: f64) -> f64 {
    let (x2, y2) = to_centered(x, y);
    x2 * y2
}

fn thingamabob(x: f64, y: f64) -> f64 {
    let (x2, y2) = to_centered(x, y);
    let (r, _) = to_polar(x2, y2);

    let r_squared = r * r;
    let r_cubed = r_squared * r;
    
    // Designed in Desmos
    -5.5 * r_cubed + 6.7 * r_squared - 1.4 * r + 0.5
}

fn main() {
    const SLICE_RES: u32 = 3;
    const CURVE_RES: u32 = 6;

    let mut printer = Printer::new("slicetest.ps", 3.0, false);
    let surf = thingamabob;

    printer.init();

    let slices = x_slices(surf, SLICE_RES, CURVE_RES);
    for slice in slices.iter() {
        printer.print_slice(slice);
    }

    printer.next_page();

    let slices = y_slices(surf, SLICE_RES, CURVE_RES);
    for slice in slices.iter() {
        printer.print_slice(slice);
    }

    printer.next_page();
}
