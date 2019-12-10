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

fn product(x: f64, y: f64) -> f64 {
    x * y
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
        let height = surf(x, y0);
        outline_vertices.push(Vec2(x, height));
    }
    let outline = Polyline::new(&outline_vertices, true);

    let mut slits = Vec::new();
    for x in make_intervals(slice_resolution, false).into_iter() {
        let height = surf(x, y0);
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
        let height = surf(x0, y);
        outline_vertices.push(Vec2(1.0 - y, height));
    }
    let outline = Polyline::new(&outline_vertices, true);

    let mut slits = Vec::new();
    for y in make_intervals(slice_resolution, false).into_iter() {
        let height = surf(x0, y);
        let slit = Polyline::new(&[
            Vec2(1.0 - y, 0.0),
            Vec2(1.0 - y, height / 2.0)
        ], false);

        slits.push(slit);
    }

    Slice::new(outline, slits)
}

fn main() {
    let mut printer = Printer::new("slicetest.ps", 2.0, false);

    let slices = x_slices(product, 5, 6);
    for slice in slices.iter() {
        printer.print_slice(slice);
    }

    printer.next_page();

    let slices = y_slices(product, 5, 6);
    for slice in slices.iter() {
        printer.print_slice(slice);
    }

    printer.next_page();
}
