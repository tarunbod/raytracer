mod math;
mod render;

use math::{Vec3, Point, Color};
use render::{RenderConfig, prim::{Sphere, Tri}, model::Model, Hittable};

fn main() {
    let mut model = Model::load_obj("teapot.obj");
    model.translate(Vec3::new(0.0, -1.8, -10.0));

    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(model),
        Box::new(Tri::new(Point::new(-10.0, -2.0, 0.0), Point::new(10.0, -2.0, 0.0), Point::new(-70.0, -2.0, -30.0), Vec3::new(0.0, 1.0, 0.0), Color::rgb(255, 255, 255))),
        Box::new(Tri::new(Point::new(-70.0, -2.0, -30.0), Point::new(10.0, -2.0, 0.0), Point::new(70.0, -2.0, -30.0), Vec3::new(0.0, 1.0, 0.0), Color::rgb(255, 255, 255))),
        Box::new(Sphere::new(Point::new(-5.0, -0.25, -10.0), 1.75, Color::rgb(60, 51, 230))),
        Box::new(Sphere::new(Point::new(5.0, -0.25, -10.0), 1.75, Color::rgb(60, 255, 230)))
    ];
    render::render(objs, RenderConfig {
        width: 1024,
        height: 576,
        samples: 128,
        bounces: 50,
        threads: Some(4),
        progress: true
    }, "teapot.png");
}
