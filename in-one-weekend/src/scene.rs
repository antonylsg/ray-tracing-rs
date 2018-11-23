use crate::camera::Camera;
use crate::hit::Hit;
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
        if let Some(record) = self.hitables.hit(1e-6, std::f64::INFINITY, &ray) {
            if let (true, Some(scattered)) = (ray.is_active(), record.scatter(ray)) {
                let color = self.color(scattered.ray);
                return scattered.attenuation.component_mul(&color);
            }

            return Vec3::zeros();
        }

        Self::background(&ray)
    }

    pub fn sample(&self, camera: &Camera, u: f64, v: f64) -> Vec3
    where
        T: Hit,
    {
        let ray = camera.cast(u, v);
        self.color(ray)
    }
}

impl Scene<Sphere> {
    pub fn random() -> Self {
        const GROUND: f64 = 1_000.0;
        const RADIUS: f64 = 0.2;

        let ground = Sphere::new(
            Vec3::new(0.0, -GROUND, 0.0),
            GROUND,
            Lambertian::new(Vec3::new(0.5, 0.5, 0.5)).boxed(),
        );

        let centers = vec![
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-4.0, 1.0, 0.0),
            Vec3::new(4.0, 1.0, 0.0),
        ];

        let radii = vec![1.0, 1.0, 1.0];

        let materials = vec![
            Dielectric::new(1.5).boxed(),
            Lambertian::new(Vec3::new(0.4, 0.2, 0.1)).boxed(),
            Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0).boxed(),
        ];

        let spheres: Vec<Sphere> = itertools::multizip((centers, radii, materials))
            .map(|(center, radius, material)| Sphere::new(center, radius, material))
            .collect();

        let mut hitables: Vec<Sphere> = (-11..11)
            .map(f64::from)
            .flat_map(|a| {
                let spheres = &spheres;
                (-11..11).map(f64::from).map(move |b| loop {
                    let x = a + 0.9 * rand::random::<f64>();
                    let z = b + 0.9 * rand::random::<f64>();
                    let center = Vec3::new(x, RADIUS, z);

                    let sphere = Sphere::new(center, RADIUS, material::random());
                    if !spheres.intersect(&sphere) {
                        break sphere;
                    }
                })
            }).collect();
        hitables.extend(spheres);
        hitables
            .iter_mut()
            .for_each(|sphere| sphere.stick_to(&ground));
        hitables.push(ground);

        Scene::new(hitables)
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
                Dielectric::new(1.5).boxed(),
                Dielectric::new(1.5).boxed(),
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
