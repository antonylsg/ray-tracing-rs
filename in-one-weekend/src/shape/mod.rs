mod sphere;

pub use crate::shape::sphere::*;

pub trait Intersect<S = Self> {
    fn intersect(&self, other: &S) -> bool;
}
