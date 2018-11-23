#[macro_use]
extern crate derive_new;
extern crate itertools;
extern crate nalgebra as na;
extern crate rand;

mod camera;
mod hit;
mod image;
mod material;
mod ray;
mod scene;
mod sphere;

use crate::camera::Camera;
use crate::image::Image;
use crate::scene::Scene;

type Vec3 = na::Vector3<f64>;

fn main() {
    // let sampling = 32;
    // let sampling = 64;
    // let sampling = 128;
    // let sampling = 256;
    let sampling = 1024;
    let image = Image::new_480p();

    let origin = Vec3::new(13.0, 2.0, 3.0);
    let look_at = -Vec3::z();
    let vertical = Vec3::y();
    let fov = 20.0;
    let aperture = 0.2;
    let focus = (look_at - origin).norm();
    let camera = Camera::new(
        origin,
        look_at,
        vertical,
        fov,
        image.aspect(),
        aperture,
        focus,
    );
    let scene = Scene::random();

    print!("{}", image.par_render(&scene, &camera, sampling));
}
