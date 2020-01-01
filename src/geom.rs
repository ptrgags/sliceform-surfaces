use std::ops::Sub;

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

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /// a x b is the determinant:
    ///
    /// ```
    /// | i  j  k|   (ay * bz - az * by) +
    /// |ax ay az| = (az * bx - ax * bz) +
    /// |bx by bz|   (ax * by - ay * bx)
    /// ```
    pub fn cross(&self, other: &Self) -> Self {
        let Self(ax, ay, az) = self;
        let Self(bx, by, bz) = other;

        let x = ay * bz - az * by;
        let y = az * bx * ax * bz;
        let z = ax * by - ay * bx;

        Self(x, y, z)
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = other;

        Vec3(x2 - x1, y2 - y1, z2 - z1)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Triangle {
    fn compute_normal(&self) -> Vec3 {
        let Self(v1, v2, v3) = self;
        let ab = v2 - v1;
        let ac = v3 - v1;
        
        ab.cross(&ac)
    }
}
