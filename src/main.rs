mod polylines;
mod laplace;
mod polynomial;
mod printer;
mod slicer;
mod surfaces;
mod heights;
mod models;

use printer::Printer;
use slicer::Slicer;

fn main() {
    let surf = models::select_model("laplace_hill");
    let slicer = Slicer::new(3, 6, surf);

    let mut printer = Printer::new("slicetest.ps", 3.0, false);
    printer.print_slices(&slicer);
}
