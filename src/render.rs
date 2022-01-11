use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use rand::prelude::*;

use crate::math::*;

pub struct HitRecord {
    p: Point,
    n: Vec3,
    t: f64,
    front_face: bool
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b * half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let mut t = (-half_b - discriminant.sqrt())/a;
        if !in_range(t, t_min, t_max) {
            t = (-half_b + discriminant.sqrt())/a;
            if !in_range(t, t_min, t_max) {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center).unit();
        Some(HitRecord {
            p: point,
            n: normal,
            t,
            front_face: Vec3::dot(&ray.direction, &normal) > 0.0
        })
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp: Option<HitRecord> = None;
        let mut closest = t_max;
        for object in self.iter() {
            let record = object.hit(ray, t_min, closest);
            if record.is_some() {
                closest = record.as_ref().unwrap().t;
                temp = record;
            }
        }
        temp
    }
}

struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    top_left_corner: Point
}

impl Camera {
    fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let horizontal = Vec3::new(viewport_height * aspect_ratio, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        Camera {
            origin: Point::ORIGIN,
            horizontal,
            vertical,
            top_left_corner: Point::ORIGIN - horizontal/2.0 + vertical/2.0 - Vec3::new(0.0, 0.0, focal_length)
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.top_left_corner + u * self.horizontal - v * self.vertical)
    }
}

fn cast_ray(r: Ray, objects: &Vec<Box<dyn Hittable>>, bounces: u32) -> Color {
    if bounces == 0 {
        return Color::ORIGIN;
    }
    if let Some(record) = objects.hit(&r, 0.001, f64::MAX) {
        let target = record.p + record.n + random_unit_vec();
        return 0.5 * cast_ray(Ray::new(record.p, target), objects, bounces - 1);
    }
    let t = (r.direction.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_png(filepath: &str, data: &[u8], width: u32, height: u32) {
    println!("Writing PNG image {}...", filepath);
    let path = Path::new(filepath);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
    println!("Done.");
}

// fn write_ppm(filepath: &str, data: &[u8], width: u32, height: u32) {
//     println!("Writing PPM image {}...", filepath);
//     let path = Path::new(filepath);
//     let file = File::create(path).unwrap();
//     let ref mut w = BufWriter::new(file);
//     w.write_fmt(format_args!("P3\n{} {}\n255\n", width, height)).unwrap();
//     for i in 0..height {
//         for j in 0..width {
//             let base_idx = ((i * width + j) * 3) as usize;
//             w.write_fmt(format_args!("{} {} {}\n", data[base_idx + 0], data[base_idx + 1], data[base_idx + 2])).unwrap();
//         }
//     }
//     println!("Done.");
// }

pub struct RenderConfig {
    pub samples: u32,
    pub bounces: u32
}

pub fn render(objects: Vec<Box<dyn Hittable>>, config: RenderConfig, filename: &str) {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut data = vec![0 as u8; image_width * image_height * 3];

    let camera = Camera::new(aspect_ratio, 2.0, 1.0);
    let mut rng = rand::thread_rng();
    
    for i in 0..image_height {
        for j in 0..image_width {
            let mut color = Color::origin();
            for _ in 0..config.samples {
                let u = ((j as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((i as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;
                color += cast_ray(
                    camera.get_ray(u, v),
                    &objects,
                    config.bounces
                );
            }
            color /= config.samples as f64;

            // color.x = clamp(color.x, 0.0, 1.0);
            // color.y = clamp(color.y, 0.0, 1.0);
            // color.z = clamp(color.z, 0.0, 1.0);

            color = color.gamma_correct(2.0);
            color *= 255.0;
            
            let base_idx = (i * image_width + j) * 3;
            data[base_idx + 0] = color.x as u8;
            data[base_idx + 1] = color.y as u8;
            data[base_idx + 2] = color.z as u8;
        }
        println!("{:.2}", i as f64 / image_height as f64 * 100.0);
    }
    write_png(filename, &data, image_width as u32, image_height as u32);
    // write_ppm(filename, &data, image_width as u32, image_height as u32);
}
