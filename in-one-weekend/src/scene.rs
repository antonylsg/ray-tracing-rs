use derive_new::new;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::image::Pixel;
use crate::material;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::ray::Ray;
use crate::shape::Intersect;
use crate::shape::Sphere;
use crate::Vec3;

#[derive(new)]
pub struct Scene<T> {
    hitables: Vec<T>,
}

impl<T> Scene<T> {
    fn background(ray: &Ray) -> Vec3 {
        let t = 0.5 * (1.0 + ray.direction.y);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);

        (1.0 - t) * white + t * blue
    }

    fn color(&self, ray: Ray) -> Vec3
    where
        T: Hit,
    {
        if let Some(impact) = self.hitables.hit(1e-6, std::f64::INFINITY, &ray) {
            if let (true, Some(scattered)) = (ray.is_active(), impact.scatter(ray)) {
                let color = self.color(scattered.ray);
                return scattered.attenuation.component_mul(&color);
            }

            return Vec3::zeros();
        }

        Self::background(&ray)
    }

    pub fn sample(&self, camera: &Camera, pixel: Pixel) -> Vec3
    where
        T: Hit,
    {
        let ray = camera.gather(pixel);
        self.color(ray)
    }
}

impl Scene<Sphere> {
    pub fn random() -> Self {
        /// Ball radius
        const BALL: f64 = 1.0;
        /// Ground radius
        const GROUND: f64 = 1_000.0;
        /// Marble radius
        const MARBLE: f64 = 0.2;

        let ground = Sphere::new(
            Vec3::new(0.0, -GROUND, 0.0),
            GROUND,
            Lambertian::new(Vec3::new(0.5, 0.5, 0.5)).boxed(),
        );

        let centers = vec![
            Vec3::new(-4.0, BALL, 0.0),
            Vec3::new(0.0, BALL, 0.0),
            Vec3::new(4.0, BALL, 0.0),
        ];

        let materials = vec![
            Lambertian::new(Vec3::new(0.4, 0.2, 0.1)).boxed(),
            Dielectric::new(Vec3::new(1.0, 1.0, 1.0), 1.5).boxed(),
            Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0).boxed(),
        ];

        let mut spheres: Vec<Sphere> = itertools::multizip((centers, materials))
            .map(|(center, material)| Sphere::new(center, BALL, material))
            .map(|sphere| sphere.stick_to(&ground))
            .collect();

        for a in (-11..11).map(f64::from) {
            for b in (-11..11).map(f64::from) {
                loop {
                    let x = a + 0.9 * rand::random::<f64>();
                    let z = b + 0.9 * rand::random::<f64>();
                    let center = Vec3::new(x, MARBLE, z);

                    let sphere = Sphere::new(center, MARBLE, material::random());
                    if !spheres.intersect(&sphere) {
                        spheres.push(sphere.stick_to(&ground));
                        break;
                    }
                }
            }
        }

        spheres.push(ground);

        Scene::new(spheres)
    }

    #[allow(dead_code)]
    pub fn test() -> Self {
        let hitables: Vec<_> = {
            let centers = vec![
                -Vec3::z(),
                Vec3::new(0.0, -100.5, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::z(),
                // Vec3::new(-3.0, 0.0, -2.0),
            ];

            let radii = vec![
                // 0.5, 100.0, 0.5, 0.5, 0.5,
                0.5, 100.0, 0.5, 0.5, -0.499, 0.5,
                // 1.0,
            ];

            let materials = vec![
                Lambertian::new(Vec3::new(0.8, 0.3, 0.3)).boxed(),
                Lambertian::new(Vec3::new(0.8, 0.8, 0.0)).boxed(),
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0).boxed(),
                // Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0).boxed(),
                Dielectric::new(Vec3::new(1.0, 1.0, 1.0), 1.5).boxed(),
                Dielectric::new(Vec3::new(1.0, 1.0, 1.0), 1.5).boxed(),
                Metal::new(Vec3::new(0.3, 0.3, 0.8), 0.5).boxed(),
                // Metal::new(Vec3::new(224.0 / 255.0, 232.0 / 255.0, 222.0 / 255.0), 0.3).boxed(),
            ];

            itertools::multizip((centers, radii, materials))
                .map(|(center, radius, material)| Sphere::new(center, radius, material))
                .collect()
        };

        Scene::new(hitables)
    }
}
