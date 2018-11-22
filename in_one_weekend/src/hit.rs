use crate::material::Material;
use crate::material::Scattered;
use crate::ray::Ray;
use crate::Vec3;

use std::cmp::Ordering;

pub trait Hit {
    fn boxed(self) -> Box<dyn Hit>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn hit(&self, min: f64, max: f64, ray: &Ray) -> Option<Record>;
}

#[derive(new)]
pub struct Record<'m> {
    parameter: f64,
    pub point: Vec3,
    pub normal: Vec3,
    material: &'m dyn Material,
}

impl<'m> Record<'m> {
    pub fn scatter(&self, ray: Ray) -> Option<Scattered> {
        self.material.scatter(ray, self)
    }
}

impl<T> Hit for Box<T>
where
    T: Hit,
{
    fn hit(&self, min: f64, max: f64, ray: &Ray) -> Option<Record> {
        (**self).hit(min, max, ray)
    }
}

impl<T> Hit for Vec<T>
where
    T: Hit,
{
    fn hit(&self, min: f64, max: f64, ray: &Ray) -> Option<Record> {
        self.iter()
            .filter_map(|hitable| hitable.hit(min, max, ray))
            .min_by(|a, b| {
                a.parameter
                    .partial_cmp(&b.parameter)
                    .unwrap_or(Ordering::Equal)
            })
    }
}
