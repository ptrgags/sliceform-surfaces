mod polylines;

use std::fs::File;

use polylines::{Vec2, Polyline};

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

fn main() {
    println!("{:?}", make_intervals(3, true));
    println!("{:?}", make_intervals(3, false));

    let points = vec![Vec2(10.0, 10.0), Vec2(100.0, 10.0), Vec2(100.0, 100.0)];
    let poly = Polyline::new(&points, false);

    let mut file = File::create("test.ps").expect("Could not open file");
    poly.write_postscript(&mut file);
}
