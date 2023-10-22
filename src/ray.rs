use crate::{Color, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self) -> Color {
        let unit_dir = self.direction.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
    }
}
