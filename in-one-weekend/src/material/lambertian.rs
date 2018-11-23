use crate::hit;
use crate::material::Material;
use crate::material::Scattered;
use crate::ray::Ray;
use crate::sphere;
use crate::Vec3;

#[derive(new)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, record: &hit::Record) -> Option<Scattered> {
        let direction = record.normal + sphere::random_inside();
        let ray = ray.next(record.impact, direction);

        Some(Scattered::new(ray, self.albedo))
    }
}
