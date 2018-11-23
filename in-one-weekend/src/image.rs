extern crate png;
extern crate rayon;

use self::png::HasParameters;
use self::rayon::prelude::*;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::na;
use crate::scene::Scene;
use crate::Vec3;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseFormatError;

#[derive(Clone, Copy)]
pub enum Format {
    Png,
    Ppm,
}

impl Format {
    fn as_str(&self) -> &str {
        match self {
            Format::Png => "png",
            Format::Ppm => "ppm",
        }
    }
}

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(Format::Png),
            "ppm" => Ok(Format::Ppm),
            _ => Err(ParseFormatError),
        }
    }
}

impl fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse format which is neither png, nor ppm")
    }
}

#[derive(new)]
pub struct Image {
    width: u32,
    height: u32,
    #[new(default)]
    buffer: Vec<u8>,
}

impl Image {
    /// 720×480 (3:2)
    #[allow(dead_code)]
    pub fn new_480p() -> Image {
        Image::new(720, 480)
    }

    /// 1280×720 (16:9) or HD Ready
    #[allow(dead_code)]
    pub fn new_720p() -> Image {
        Image::new(1280, 720)
    }

    /// 1920×1080 (16:9) or Full HD
    #[allow(dead_code)]
    pub fn new_1080p() -> Image {
        Image::new(1920, 1080)
    }

    /// 3840×2160 (16:9) or Ultra HD
    #[allow(dead_code)]
    pub fn new_2160p() -> Image {
        Image::new(3840, 2160)
    }

    pub fn aspect(&self) -> f64 {
        f64::from(self.width) / f64::from(self.height)
    }

    pub fn par_render<T>(&mut self, scene: &Scene<T>, camera: &Camera, sampling: u32)
    where
        T: Hit + Sync,
    {
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
                            scene.sample(camera, u, v)
                        }).sum();

                    let mut color = color / f64::from(sampling);
                    // Gamma correction
                    color.apply(f64::sqrt);
                    let color: na::Vector3<u8> = na::try_convert(255.0 * color).unwrap();

                    vec![color.x, color.y, color.z]
                })
            }).flatten()
            .collect();

        self.buffer.extend(body);
    }

    fn save_as_png(&self, writer: impl Write) -> Result<(), png::EncodingError> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

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
            .join("image")
            .with_extension(format.as_str());
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);

        match format {
            Format::Png => self.save_as_png(writer),
            Format::Ppm => self.save_as_ppm(writer).map_err(|err| err.into()),
        }
    }
}
