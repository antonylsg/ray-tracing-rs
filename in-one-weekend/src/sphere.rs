use crate::hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;

#[derive(new)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl hit::Hit for Sphere {
    fn hit(&self, min: f64, max: f64, ray: &Ray) -> Option<hit::Record> {
        let centered = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        // 2.0 cancels out
        let b = ray.direction.dot(&centered);
        let c = centered.norm_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant.is_sign_negative() {
            return None;
        }

        let sqrt = discriminant.sqrt();

        let root = (-b - sqrt) / a;
        if min <= root && root <= max {
            let point = ray.point_at(root);
            let normal = (point - self.center) / self.radius;
            let material = self.material.as_ref();
            return Some(hit::Record::new(root, point, normal, material));
        }

        let root = (-b + sqrt) / a;
        if min <= root && root <= max {
            let point = ray.point_at(root);
            let normal = (point - self.center) / self.radius;
            let material = self.material.as_ref();
            return Some(hit::Record::new(root, point, normal, material));
        }

        None
    }
}

pub fn random_inside() -> Vec3 {
    loop {
        let random = Vec3::new(rand::random(), rand::random(), rand::random());
        let random = 2.0 * random - Vec3::new(1.0, 1.0, 1.0);
        if random.norm_squared() < 1.0 {
            break random;
        }
    }
}
