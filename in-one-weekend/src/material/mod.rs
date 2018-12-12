use derive_new::new;

use crate::hit;
use crate::ray::Ray;
use crate::Vec3;

mod dielectric;
mod lambertian;
mod metal;

pub use crate::material::dielectric::*;
pub use crate::material::lambertian::*;
pub use crate::material::metal::*;

pub trait Material: Send + Sync {
    fn boxed(self) -> Box<dyn Material>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn scatter(&self, ray: Ray, impact: &hit::Impact<'_>) -> Option<Scattered>;
}

#[derive(new)]
pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub fn random() -> Box<dyn Material> {
    let random = rand::random::<f64>();
    if random < 0.8 {
        let x = rand::random::<f64>() * rand::random::<f64>();
        let y = rand::random::<f64>() * rand::random::<f64>();
        let z = rand::random::<f64>() * rand::random::<f64>();

        Lambertian::new(Vec3::new(x, y, z)).boxed()
    } else if random < 0.95 {
        let x = 0.5 * (1.0 + rand::random::<f64>());
        let y = 0.5 * (1.0 + rand::random::<f64>());
        let z = 0.5 * (1.0 + rand::random::<f64>());
        let fuzz = 0.5 * rand::random::<f64>();

        Metal::new(Vec3::new(x, y, z), fuzz).boxed()
    } else {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Dielectric::new(attenuation, 1.5).boxed()
    }
}
