mod math;
mod render;

use math::{Vec3, Point, Color};
use render::{RenderConfig, prim::{Sphere, Tri}, Hittable};

fn get_tri(p1: Point, p2: Point, p3: Point) -> Tri {
    let n = Vec3::cross(&(p2 - p1), &(p3 - p1));
    Tri::new(
        p1, p2, p3, n,
        n.unit()
    )
}

fn main() {
    // let objs: Vec<Box<dyn Hittable>> = vec![
    //     // Box::new(Sphere::new(Point::new(0.0, -100.25, -1.0), 100.0, Color::rgb(230, 50, 60)))
    //     Box::new(get_tri(
    //         Point::new(0.0, 0.0, -5.0),
    //         Point::new(1.0, 0.0, -4.0),
    //         Point::new(0.0, 1.0, -4.0)
    //     )),
    //     Box::new(get_tri(
    //         Point::new(0.0, 0.0, -5.0),
    //         Point::new(-1.0, 0.0, -4.0),
    //         Point::new(0.0, 1.0, -4.0),
    //     )),
    //     Box::new(get_tri(
    //         Point::new(0.0, 0.0, -5.0),
    //         Point::new(1.0, 0.0, -4.0),
    //         Point::new(0.0, -1.0, -4.0),
    //     )),
    //     Box::new(get_tri(
    //         Point::new(0.0, 0.0, -5.0),
    //         Point::new(-1.0, 0.0, -4.0),
    //         Point::new(0.0, -1.0, -4.0),
    //     ))
    // ];
    
    let mut objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0.0, -100.25, -1.0), 100.0, Color::rgb(230, 50, 60)))
    ];
    for i in 0..5 {
        objs.push(Box::new(Sphere::new(Point::new(-1.0 + (i as f64) / 2.0, 0.0, -1.0), 0.25, Color::rgb(60, 51 * i, 230))));
    }
    render::render_multi(objs, RenderConfig {
        width: 1920,
        height: 1080,
        samples: 100,
        bounces: 50
    }, "test.png");
}
