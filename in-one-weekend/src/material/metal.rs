use crate::hit;
use crate::material::Material;
use crate::material::Scattered;
use crate::ray::Ray;
use crate::shape;
use crate::Vec3;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        assert!(0.0 <= fuzz && fuzz <= 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, impact: &hit::Impact) -> Option<Scattered> {
        let reflected = reflect(&ray.direction, &impact.normal);
        let fuzzed = reflected + self.fuzz * shape::random_in_unit_sphere();

        if fuzzed.dot(&impact.normal).is_sign_negative() {
            return None;
        }

        let ray = ray.next(impact.point, fuzzed);

        Some(Scattered::new(ray, self.albedo))
    }
}

pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}
