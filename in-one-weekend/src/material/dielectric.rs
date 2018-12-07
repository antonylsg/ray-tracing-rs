use derive_new::new;
use rand::Rng;

use crate::hit;
use crate::material;
use crate::material::Material;
use crate::material::Scattered;
use crate::ray::Ray;
use crate::Vec3;

#[derive(new)]
pub struct Dielectric {
    /// Index of refraction.
    index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, impact: &hit::Impact<'_>) -> Option<Scattered> {
        let normal;
        let ratio;
        let cosine;

        let dot = ray.direction.dot(&impact.normal);
        if dot.is_sign_negative() {
            normal = impact.normal;
            ratio = self.index.recip();
            cosine = -dot;
        } else {
            normal = -impact.normal;
            ratio = self.index;
            cosine = dot;
        }

        let direction = refract(&ray.direction, &normal, ratio)
            .filter(|_| !rand::thread_rng().gen_bool(schlick(cosine, self.index)))
            .unwrap_or_else(|| material::reflect(&ray.direction, &normal));
        let ray = ray.next(impact.point, direction);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some(Scattered::new(ray, attenuation))
    }
}

/// `ratio` is the ratio of n_incident over n_transmitted.
/// `incident` must be normalized.
fn refract(incident: &Vec3, normal: &Vec3, ratio: f64) -> Option<Vec3> {
    let cosine = incident.dot(normal);
    let discriminant = 1.0 - ratio.powi(2) * (1.0 - cosine.powi(2));

    if discriminant.is_sign_negative() {
        return None;
    }

    let refracted = ratio * incident - (ratio * cosine + discriminant.sqrt()) * normal;

    Some(refracted)
}

/// The probability to not be refracted.
fn schlick(cosine: f64, index: f64) -> f64 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0.powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
