mod math;
mod render;

use math::{Vec3, Point, Color};
use render::{
    RenderConfig,
    CameraConfig,
    prim::{Sphere, Rect},
    model::Model,
    Hittable,
    Renderer
};

fn main() {
    let mut model = Model::load_obj("suzanne.obj");
    model.translate(Vec3::new(0.0, 0.0, -2.0));

    // let floor = Rect::new(Point::new(-1.0, 0.0, -9.0), Point::new(1.0, 0.0, -9.0), Point::new(-1.0, 0.0, -11.0), Point::new(1.0, 0.0, -11.0), Vec3::new(0.0, 1.0, 0.0), Color::rgb(255, 0, 0));
    // println!("{:?}", floor);    

    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(model),
    //     Box::new(floor),
    //     Box::new(Sphere::new(Point::new(-5.0, 1.75, -10.0), 1.75, Color::rgb(60, 51, 230))),
    //     Box::new(Sphere::new(Point::new(5.0, 1.75, -10.0), 1.75, Color::rgb(60, 255, 230)))
    ];
    let mut config = RenderConfig {
        width: 480,
        height: 270,
        samples: 10,
        bounces: 10,
        threads: Some(4),
        camera: CameraConfig {
            pos: Point::new(0.0, 0.0, 4.0),
            lookat: Point::new(0.0, 0.0, -1.0),
            fov: 70.0
        },
        progress: true
    };

    let renderer = Renderer::new(objs);
    for i in 70..=90 {
        config.camera.fov = i as f64;
        renderer.render(config, format!("output/suzanne{i}.png"));
    }
}
