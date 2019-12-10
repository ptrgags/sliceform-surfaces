#[derive(Debug, Copy, Clone)]
pub struct Vec2(f64, f64);

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
}
