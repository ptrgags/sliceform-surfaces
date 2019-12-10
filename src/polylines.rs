use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn translate(&self, amount: Self) -> Self {
        let Self(x, y) = self;
        let Self(dx, dy) = amount;

        Self(x + dx, y + dy)
    }

    pub fn scale(&self, factor: f64) -> Self {
        let Self(x, y) = self;

        Self(x * factor, y * factor)
    }
}

pub struct BBox {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64
}

impl BBox {
    pub fn new() -> Self {
        Self {
            left: std::f64::INFINITY,
            right: std::f64::NEG_INFINITY,
            top: std::f64::NEG_INFINITY,
            bottom: std::f64::INFINITY,
        }
    }

    pub fn add_point(&mut self, point: &Vec2) {
        let Vec2(x, y) = point;

        if *x < self.left {
            self.left = *x;
        }

        if *x > self.right {
            self.right = *x;
        }

        if *y < self.bottom {
            self.bottom = *y
        }

        if *y > self.top {
            self.top = *y
        }
    }

    pub fn width(&self) -> f64 {
        return self.right - self.left;
    }

    pub fn height(&self) -> f64 {
        return self.top - self.bottom;
    }

    pub fn get_outline(&self) -> Polyline {
        Polyline::new(&[
            Vec2(self.left, self.bottom),
            Vec2(self.right, self.bottom),
            Vec2(self.right, self.top),
            Vec2(self.left, self.top),
        ], true)
    }
}

pub struct Polyline {
    vertices: Vec<Vec2>
}

impl Polyline {
    pub fn new(vertices: &[Vec2], closed: bool) -> Self {
        let mut points = vertices.to_vec();
        if closed {
            points.push(points[0]);
        }

        Self {
            vertices: points
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        let vertices = self.vertices
            .iter()
            .map(|v| v.scale(factor))
            .collect();

        Self {
            vertices
        }
    }

    pub fn translate(&self, amount: Vec2) -> Self {
        let vertices = self.vertices
            .iter()
            .map(|v| v.translate(amount))
            .collect();
        
        Self {
            vertices
        }
    }

    pub fn get_bbox(&self) -> BBox {
        let mut bbox = BBox::new();
        for v in self.vertices.iter() {
            bbox.add_point(v);
        }

        bbox
    }

    pub fn write_postscript(&self, file: &mut File) {
        // Don't write individual points.
        if self.vertices.len() < 2 {
            return;
        }

        writeln!(file, "newpath").expect("Failed to write newpath");

        let Vec2(x, y) = self.vertices[0]; 
        writeln!(file, "{} {} moveto", x, y).expect("failed to write moveto");

        for vertex in self.vertices[1..].iter() {
            let Vec2(x, y) = vertex;
            writeln!(file, "{} {} lineto", x, y)
                .expect("failed to write lineto");
        }
        writeln!(file, "stroke").expect("failed to write stroke");
    }
}

pub struct Slice {
    outline: Polyline,
    slits: Vec<Polyline>,
}

impl Slice {
    pub fn new(outline: Polyline, slits: Vec<Polyline>) -> Self {

        Self {
            outline,
            slits,
        }
    }

    pub fn get_bbox(&self) -> BBox {
        self.outline.get_bbox()
    }

    pub fn translate(&self, amount: Vec2) -> Self {
        let outline = self.outline.translate(amount);
        let slits = self.slits
            .iter()
            .map(|slit| slit.translate(amount))
            .collect();

        Self::new(outline, slits) 
    }

    pub fn scale(&self, factor: f64) -> Self {
        let outline = self.outline.scale(factor);
        let slits = self.slits
            .iter()
            .map(|slit| slit.scale(factor))
            .collect();

        Self::new(outline, slits) 
    }

    pub fn write_postscript(&self, file: &mut File, draw_bbox: bool) {
        if draw_bbox {
            self.outline.get_bbox().get_outline().write_postscript(file);
        }

        self.outline.write_postscript(file);
        for slit in self.slits.iter() {
            slit.write_postscript(file);
        }
    }
}
