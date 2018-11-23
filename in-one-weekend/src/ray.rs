use crate::Vec3;

const MAX_DEPTH: usize = 64;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    depth: usize,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
            depth: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.depth < MAX_DEPTH
    }

    pub fn next(self, origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
            depth: self.depth + 1,
        }
    }

    pub fn impact_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
