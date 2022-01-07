mod math;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use math::*;

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

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn cast_ray(r: Ray) -> Color {
    // let unit_dir = r.direction.unit();
    if hit_sphere(Point::new(0.0,0.0,-3.0), 0.5, &r) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let t = (r.direction.y + 1.0) * 0.5;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0) 
    }
}

fn render() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 480;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut data = vec![0 as u8; image_width * image_height * 3];

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = Point::ORIGIN - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    for i in 0..image_height {
        for j in 0..image_width {
            let u = j as f64 / (image_width - 1) as f64;
            let v = i as f64 / (image_height - 1) as f64;
            let color = cast_ray(Ray::new(Point::ORIGIN, lower_left_corner + u * horizontal + v * vertical));
            data[((image_height - i - 1) * image_width + j) * 3 + 0] = (255.0 * color.x) as u8;
            data[((image_height - i - 1) * image_width + j) * 3 + 1] = (255.0 * color.y) as u8;
            data[((image_height - i - 1) * image_width + j) * 3 + 2] = (255.0 * color.z) as u8;
        }
    }
    write_png("test.png", &data, image_width as u32, image_height as u32);
}

fn main() {
    render();
}
