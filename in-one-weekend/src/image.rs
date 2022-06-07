use rayon::prelude::*;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::na;
use crate::scene::Scene;
use crate::Vec3;

use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub type Pixel = na::Vector2<f64>;

#[derive(Clone, Copy, EnumString, Display)]
pub enum Format {
    #[strum(serialize = "png")]
    Png,
    #[strum(serialize = "ppm")]
    Ppm,
}

#[derive(Clone, Copy, EnumString)]
pub enum Resolution {
    /// 720×480 (3:2)
    #[strum(serialize = "480p", serialize = "480")]
    P480,
    /// 1280×720 (16:9) or HD Ready
    #[strum(serialize = "720p", serialize = "720")]
    P720,
    /// 1920×1080 (16:9) or Full HD
    #[strum(serialize = "1080p", serialize = "1080")]
    P1080,
    /// 3840×2160 (16:9) or Ultra HD
    #[strum(serialize = "2160p", serialize = "2160")]
    P2160,
}

pub struct Image {
    width: u32,
    height: u32,
    sampling: u32,
    buffer: Vec<u8>,
}

impl Image {
    pub fn new(resolution: Resolution, sampling: u32) -> Image {
        let (width, height) = match resolution {
            Resolution::P480 => (720, 480),
            Resolution::P720 => (1280, 720),
            Resolution::P1080 => (1920, 1080),
            Resolution::P2160 => (3840, 2160),
        };

        Image {
            width,
            height,
            sampling,
            buffer: Vec::new(),
        }
    }

    pub fn aspect(&self) -> f64 {
        f64::from(self.width) / f64::from(self.height)
    }

    pub fn par_render<T>(&mut self, scene: &Scene<T>, camera: &Camera)
    where
        T: Hit + Sync,
    {
        let sampling = self.sampling;
        let body: Vec<u8> = (0..self.height)
            .into_par_iter()
            .rev()
            .flat_map(|j| {
                let width = f64::from(self.width - 1);
                let height = f64::from(self.height - 1);

                (0..self.width).into_par_iter().map(move |i| {
                    let color: Vec3 = (0..sampling)
                        .map(|_| {
                            let u = (f64::from(i) + rand::random::<f64>()) / width;
                            let v = (f64::from(j) + rand::random::<f64>()) / height;
                            scene.sample(camera, Pixel::new(u, v))
                        })
                        .sum();

                    let mut color = color / f64::from(sampling);
                    // Gamma correction
                    color.apply(|x| *x = x.sqrt());
                    let color: na::Vector3<u8> = na::try_convert(255.0 * color).unwrap();

                    vec![color.x, color.y, color.z]
                })
            })
            .flatten()
            .collect();

        self.buffer.extend(body);
    }

    fn save_as_png(&self, writer: impl Write) -> Result<(), png::EncodingError> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.buffer)
    }

    fn save_as_ppm(&self, mut writer: impl Write) -> io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;
        self.buffer
            .chunks(3)
            .try_for_each(|chunk| writeln!(writer, "{} {} {}", chunk[0], chunk[1], chunk[2]))
    }

    pub fn save_as(&self, format: Format) -> Result<(), png::EncodingError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("{}p@{}", self.height, self.sampling))
            .with_extension(format.to_string());
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);

        match format {
            Format::Png => self.save_as_png(writer),
            Format::Ppm => self.save_as_ppm(writer).map_err(Into::into),
        }
    }
}
