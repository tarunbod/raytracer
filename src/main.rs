mod math;
mod render;

use math::{Point, Color};
use render::{RenderConfig, Sphere, Hittable};

fn main() {
    let mut objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0.0, -100.25, -1.0), 100.0, Color::rgb(230, 50, 60)))
    ];
    for i in 0..5 {
        objs.push(Box::new(Sphere::new(Point::new(-1.0 + (i as f64) / 2.0, 0.0, -1.0), 0.25, Color::rgb(60, 51 * i, 230))));
    }
    render::render(objs, RenderConfig {
        samples: 100,
        bounces: 50
    }, "test.png");
}
