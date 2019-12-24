mod polylines;
mod polynomial;
mod printer;
mod slicer;
mod surfaces;
mod heights;
mod models;

use printer::Printer;
use slicer::Slicer;

fn main() {
    let surf = models::select_model("sine_hill");
    let slicer = Slicer::new(2, 6, surf);

    let mut printer = Printer::new("slicetest.ps", 3.5, false);
    printer.print_slices(&slicer);
}
