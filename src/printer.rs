use std::fs::File;
use std::io::Write;

use crate::polylines::{Vec2, BBox, Slice};

pub struct Printer {
    cursor: Vec2,
    file: File,
    scale: f64,
    draw_bboxes: bool
}

impl Printer {
    // 72 points per inch
    const INCH: f64 = 72.0;

    // Letter-sized paper with 1/2 inch margins
    const WIDTH: f64 = 8.5 * Self::INCH;
    const HEIGHT: f64 = 11.0 * Self::INCH;
    const MARGIN: f64 = 0.5 * Self::INCH;

    pub fn new(fname: &str, scale: f64, draw_bboxes: bool) -> Self {
        let file = File::create(fname).expect("Failed to open output file");

        Self {
            cursor: Vec2(Self::MARGIN, Self::MARGIN),
            file,
            scale,
            draw_bboxes
        }
    }

    fn next_row(&mut self, bbox: &BBox) {
        self.cursor = self.cursor.translate(Vec2(0.0, bbox.height()));
    }

    fn next_column(&mut self, bbox: &BBox) {
        let Vec2(x, _) = self.cursor;
        self.cursor = Vec2(x + bbox.width(), Self::MARGIN);
    }

    pub fn next_page(&mut self) {
        writeln!(self.file, "showpage").expect("could not write showpage");
        //writeln!(self.file, "0 0 1 setrgbcolor").expect("Could not set color");
        self.cursor = Vec2(Self::MARGIN, Self::MARGIN);
    }

    fn draw_slice_at_cursor(&mut self, slice: &Slice) {
        slice.translate(self.cursor)
            .write_postscript(&mut self.file, self.draw_bboxes);

        self.next_row(&slice.get_bbox());
    }

    pub fn init(&mut self) {
        //writeln!(self.file, "0 0 1 setrgbcolor").expect("Could not set color");
    }

    pub fn print_slice(&mut self, slice: &Slice) {
        let scaled = slice.scale(self.scale * Self::INCH);
        let bbox = scaled.get_bbox();
        let Vec2(x, y) = self.cursor;

        if y + bbox.height() <= Self::HEIGHT - Self::MARGIN {
            self.draw_slice_at_cursor(&scaled);
        } else if x + 2.0 * bbox.width() <= Self::WIDTH - Self::MARGIN {
            self.next_column(&bbox);
            self.draw_slice_at_cursor(&scaled);
        } else {
            self.next_page();
            self.draw_slice_at_cursor(&scaled);
        }
    }
}
