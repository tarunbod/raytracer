mod math;
mod render;

use math::Point;
use render::{RenderConfig, Sphere, Hittable};

fn main() {
    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0))
    ];
    render::render(objs, RenderConfig {
        samples: 100,
        bounces: 50
    }, "test.png");
}
