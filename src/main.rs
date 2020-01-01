mod polylines;
mod polynomial;
mod printer;
mod slicer;
mod surfaces;
mod heights;
mod models;
mod preview;
mod geom;
mod mesh;

use printer::Printer;
use slicer::Slicer;
use preview::SurfacePreview;

fn main() {
    const MODEL: &str = "sine_hill";
    const SLICE_RES: u32 = 2;
    const CURVE_RES: u32 = 6;
    const SLICE_WIDTH_INCHES: f64 = 3.0;
    const DRAW_BBOXES: bool = false;
    const PS_FNAME: &str = "slicetest.ps";
    const M: usize = 50;
    const N: usize = 50;
    const P: usize = 10;
    const OBJ_FILE: &str = "preview.obj";

    let surf = models::select_model(MODEL);
    let slicer = Slicer::new(SLICE_RES, CURVE_RES, surf);
    let mut printer = Printer::new(PS_FNAME, SLICE_WIDTH_INCHES, DRAW_BBOXES);
    printer.print_slices(&slicer);

    let surf = models::select_model(MODEL);
    let mut previewer = SurfacePreview::new(M, N, P, surf);
    previewer.generate_mesh();
    previewer.save_obj_file(OBJ_FILE);
}
