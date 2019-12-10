use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn transform(&self, scale: f64, translate: Self) -> Self {
        let Self(x, y) = self;
        let Self(dx, dy) = translate;
        
        Self(scale * x + dx, scale * y + dy)
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

    pub fn transform(&self, scale: f64, translate: Vec2) -> Self {
        let vertices = self.vertices
            .iter()
            .map(|v| v.transform(scale, translate))
            .collect();
        
        Self {
            vertices
        }
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
