pub mod prim;
pub mod model;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::prelude::*;

use crate::math::*;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    diffuse: Color
}

pub struct HitRecord {
    p: Point,
    material: Material,
    n: Vec3,
    t: f64,
    front_face: bool
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    fn new(position: Point, lookat: Point, aspect_ratio: f64, fov: f64) -> Camera {
        let w = (position - lookat).unit();
        let u = Vec3::cross(&Vec3::new(0.0, 1.0, 0.0), &w).unit();
        let v = Vec3::cross(&w, &u);

        let viewport_width = 2.0 * (fov / 2.0).to_radians().tan();
        let horizontal = viewport_width * u; // Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = viewport_width / aspect_ratio * v; // Vec3::new(0.0, viewport_width / aspect_ratio, 0.0);

        Camera {
            origin: position,
            horizontal,
            vertical,
            top_left_corner: position - horizontal/2.0 + vertical/2.0 - w
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, (self.top_left_corner + u * self.horizontal - v * self.vertical) - self.origin)
    }
}

fn cast_ray(r: Ray, objects: &Vec<Box<dyn Hittable>>, bounces: u32) -> Color {
    if bounces == 0 {
        return Color::ORIGIN;
    }
    if let Some(record) = objects.hit(&r, 0.001, f64::MAX) {
        let target = random_hemisphere_vec(record.n);
        return record.material.diffuse * cast_ray(Ray::new(record.p, target), objects, bounces - 1);
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

#[derive(Clone, Copy)]
pub struct CameraConfig {
    pub pos: Point,
    pub lookat: Point,
    pub fov: f64
}

#[derive(Clone, Copy)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
    pub threads: Option<u32>,
    pub camera: CameraConfig,
    pub progress: bool
}

pub struct Renderer {
    objects: Arc<Vec<Box<dyn Hittable>>>
}

impl Renderer {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Renderer {
        Renderer {
            objects: Arc::new(objects)
        }
    }

    pub fn render(&self, config: RenderConfig, filename: String) {
        let aspect_ratio: f64 = config.width as f64 / config.height as f64;
        let pixels = Arc::new(Mutex::new(vec![0 as u8; (config.width * config.height * 3) as usize]));
        let camera = Arc::new(Camera::new(config.camera.pos, config.camera.lookat, aspect_ratio, config.camera.fov));

        let num_threads = config.threads.unwrap_or(1);
        let slice_size = config.width / num_threads;
    
        let mut handles = vec![];
        for n in 0..num_threads {
            let data = Arc::clone(&pixels);
            let my_camera = Arc::clone(&camera);
            let objs = Arc::clone(&self.objects);
            handles.push(thread::spawn(move || {
                let start = slice_size * n;
                let end = start + slice_size;
                let mut rng = rand::thread_rng();
                for j in start..end {
                    for i in 0..config.height {
                        let mut color = Color::origin();
                        for _ in 0..config.samples {
                            let u = ((j as f64) + rng.gen::<f64>()) / (config.width - 1) as f64;
                            let v = ((i as f64) + rng.gen::<f64>()) / (config.height - 1) as f64;
                            color += cast_ray(
                                my_camera.get_ray(u, v),
                                &objs,
                                config.bounces
                            );
                        }
                        color /= config.samples as f64;
            
                        color = color.gamma_correct(2.0);
                        color *= 255.0;
                        
                        let base_idx = ((i * config.width + j) * 3) as usize;
                        let mut my_data = data.lock().unwrap();
                        my_data[base_idx + 0] = color.x as u8;
                        my_data[base_idx + 1] = color.y as u8;
                        my_data[base_idx + 2] = color.z as u8;
                    }
                    if config.progress {
                        println!("[Thread {}]: {:.2}", n, (j - start) as f64 / slice_size as f64 * 100.0);
                    }
                }
            }));
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
        
        write_png(filename.as_str(), &pixels.lock().unwrap(), config.width as u32, config.height as u32);
    }
}

