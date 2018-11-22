extern crate rayon;

use self::rayon::prelude::*;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::na;
use crate::scene::Scene;
use crate::Vec3;

#[derive(new)]
pub struct Image {
    width: u32,
    height: u32,
}

impl Image {
    /// 720×480
    #[allow(dead_code)]
    pub fn new_480p() -> Image {
        Image::new(720, 480)
    }

    /// 1280×720
    #[allow(dead_code)]
    pub fn new_720p() -> Image {
        Image::new(1280, 720)
    }

    /// 1920×1080
    #[allow(dead_code)]
    pub fn new_1080p() -> Image {
        Image::new(1920, 1080)
    }

    pub fn aspect(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn par_render<T>(&self, scene: &Scene<T>, camera: &Camera, sampling: u32) -> String
    where
        T: Hit + Sync,
    {
        let header = vec![
            "P3\n".to_string(),
            format!("{} {}\n", self.width, self.height),
            "255\n".to_string(),
        ];

        let body = (0..self.height).into_par_iter().rev().flat_map(|j| {
            (0..self.width).into_par_iter().map(move |i| {
                let color: Vec3 = (0..sampling)
                    .map(|_| {
                        let u = (i as f64 + rand::random::<f64>()) / (self.width - 1) as f64;
                        let v = (j as f64 + rand::random::<f64>()) / (self.height - 1) as f64;
                        scene.sample(camera, u, v)
                    }).sum();

                let mut color = color / sampling as f64;
                // Gamma correction
                color.apply(f64::sqrt);
                let color: na::Vector3<u8> = na::try_convert(255.0 * color).unwrap();

                format!("{} {} {}\n", color.x, color.y, color.z)
            })
        });

        header.into_par_iter().chain(body).collect()
    }
}
