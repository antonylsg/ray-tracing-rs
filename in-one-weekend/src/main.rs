#[macro_use]
extern crate derive_new;
extern crate nalgebra as na;
extern crate structopt;

use structopt::StructOpt;

mod camera;
mod hit;
mod image;
mod material;
mod ray;
mod scene;
mod shape;

use crate::camera::Camera;
use crate::image::Format;
use crate::image::Image;
use crate::scene::Scene;

type Vec3 = na::Vector3<f64>;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        short,
        long,
        default_value = "png",
        help = "sets the image format to either png or ppm"
    )]
    format: Format,

    #[structopt(
        short,
        long,
        help = "sets the numbers of rays per image pixel"
    )]
    sampling: u32,
}

fn main() {
    let opt = Opt::from_args();

    let mut image = Image::new_480p();

    let origin = Vec3::new(13.0, 2.0, 3.0);
    let look_at = -Vec3::z();
    let vertical = Vec3::y();
    let fov = 20.0;
    let aperture = 0.1;
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

    image.par_render(&scene, &camera, opt.sampling);
    image.save_as(opt.format).unwrap();
}
