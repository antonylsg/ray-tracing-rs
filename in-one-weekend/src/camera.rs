use crate::ray::Ray;
use crate::Vec3;

pub struct Camera {
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// `vertical_fov` in degrees.
    /// `focus` distance.
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vertical: Vec3,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus: f64,
    ) -> Camera {
        let theta = 0.5 * vertical_fov.to_radians();
        let half_height = theta.tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).normalize();
        let u = vertical.cross(&w).normalize();
        let v = w.cross(&u).normalize();

        let lower_left_corner = origin - focus * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * focus * half_width * u;
        let vertical = 2.0 * focus * half_height * v;

        Camera {
            u,
            v,
            // w,
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius: 0.5 * aperture,
        }
    }

    pub fn cast(&self, u: f64, v: f64) -> Ray {
        let random = self.lens_radius * random_on_unit_disk();
        let offset = random.x * self.u + random.y * self.v;
        let origin = self.origin + offset;
        let mut direction = self.lower_left_corner;
        direction += u * self.horizontal;
        direction += v * self.vertical;
        direction -= origin;

        Ray::new(origin, direction)
    }
}

fn random_on_unit_disk() -> Vec3 {
    loop {
        let random = Vec3::new(rand::random(), rand::random(), rand::random());
        let random = 2.0 * random - Vec3::new(1.0, 1.0, 0.0);
        if random.norm_squared() < 1.0 {
            break random;
        }
    }
}
