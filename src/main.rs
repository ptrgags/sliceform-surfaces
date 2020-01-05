#[macro_use]
mod macros;
mod heights;
mod height1d;
mod xforms1d;
mod csg1d;
mod csg2d;
mod combine;
mod models;
mod polylines;
mod polynomial;
mod printer;
mod slicer;
mod surfaces;
mod preview;
mod geom;
mod mesh;

use std::env::args;
use std::process::exit;

use printer::Printer;
use slicer::Slicer;
use preview::SurfacePreview;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("usage: sliceform-surfaces model_id");
        exit(1);
    }
    let model = &args[1];
    let ps_fname = format!("models/{}.ps", model);
    let obj_fname = format!("models/{}.obj", model);

    const SLICE_RES: u32 = 2;
    const CURVE_RES: u32 = 6;
    const SLICE_WIDTH_INCHES: f64 = 3.0;
    const DRAW_BBOXES: bool = false;
    const M: usize = 100;
    const N: usize = 100;
    const P: usize = 10;

    let surf = models::select_model(model);
    let slicer = Slicer::new(SLICE_RES, CURVE_RES, surf);
    let mut printer = Printer::new(&ps_fname, SLICE_WIDTH_INCHES, DRAW_BBOXES);
    printer.print_slices(&slicer);

    let surf = models::select_model(model);
    let mut previewer = SurfacePreview::new(M, N, P, surf);
    previewer.generate_mesh();
    previewer.save_obj_file(&obj_fname);
}
