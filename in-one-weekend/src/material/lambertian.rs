use derive_new::new;

use crate::hit;
use crate::material::Material;
use crate::material::Scattered;
use crate::ray::Ray;
use crate::shape;
use crate::Vec3;

#[derive(new)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, impact: &hit::Impact<'_>) -> Option<Scattered> {
        let direction = impact.normal + shape::random_in_unit_sphere();
        let ray = ray.next(impact.point, direction);

        Some(Scattered::new(ray, self.albedo))
    }
}
