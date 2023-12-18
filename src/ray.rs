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
        let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, self);
        if t > 0.0 {
            let n = Vec3::unit_vector(&(self.at(t) - Vec3::new(0.0, 0.0, -1.0)));
            return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
        let unit_dir = self.direction.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(&oc, &r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        -half_b - discriminant.sqrt() * a
    }
}
