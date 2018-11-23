use crate::hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Intersect;
use crate::Vec3;

#[derive(new)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn stick_to(mut self, other: &Sphere) -> Self {
        if self.center == other.center {
            return self;
        }

        let direction = (self.center - other.center).normalize();
        let norm = self.radius + other.radius;
        self.center = norm * direction + other.center;
        self
    }
}

impl Intersect for Sphere {
    fn intersect(&self, other: &Sphere) -> bool {
        let distance2 = (self.center - other.center).norm_squared();
        let radii2 = (self.radius + other.radius).powi(2);

        distance2 < radii2
    }
}

impl Intersect<Sphere> for [Sphere] {
    fn intersect(&self, other: &Sphere) -> bool {
        self.iter().any(|sphere| sphere.intersect(other))
    }
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
            let impact = ray.impact_at(root);
            let normal = (impact - self.center) / self.radius;
            let material = self.material.as_ref();
            return Some(hit::Record::new(root, impact, normal, material));
        }

        let root = (-b + sqrt) / a;
        if min <= root && root <= max {
            let impact = ray.impact_at(root);
            let normal = (impact - self.center) / self.radius;
            let material = self.material.as_ref();
            return Some(hit::Record::new(root, impact, normal, material));
        }

        None
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let random = Vec3::new(rand::random(), rand::random(), rand::random());
        let random = 2.0 * random - Vec3::new(1.0, 1.0, 1.0);
        if random.norm_squared() < 1.0 {
            break random;
        }
    }
}
