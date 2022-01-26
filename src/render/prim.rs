use crate::math::*;
use crate::render::{Material, Hittable, HitRecord};

pub struct Tri {
    p1: Point,
    p2: Point,
    p3: Point,
    normal: Vec3,
    material: Material
}

impl Tri {
    pub fn new(p1: Point, p2: Point, p3: Point, normal: Vec3, base_color: Color) -> Tri {
        // let n = Vec3::cross(&(p1 - p2), &(p3 - p2)).unit();
        Tri { p1, p2, p3, normal, material: Material { diffuse: base_color } }
    }

    pub fn contains(&self, p: Point) -> bool {
        let e0 = self.p2 - self.p1;
        let vp0 = p - self.p1; 
        let mut c = Vec3::cross(&e0, &vp0); 
        if Vec3::dot(&self.normal, &c) < 0.0 {
             return false;
        }
    
        let e1 = self.p3 - self.p2; 
        let vp1 = p - self.p2; 
        c = Vec3::cross(&e1, &vp1); 
        if Vec3::dot(&self.normal, &c) < 0.0 {
            return false; // P is on the right side 
        }
    
        let e2 = self.p1 - self.p3; 
        let vp2 = p - self.p3; 
        c = Vec3::cross(&e2, &vp2); 
        if Vec3::dot(&self.normal, &c) < 0.0 {
            return false;
        }

        return true;
    }
}

impl Hittable for Tri {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let d = Vec3::dot(&self.normal, &self.p1);
        let t = Vec3::dot(&self.normal, &ray.origin) + d / Vec3::dot(&self.normal, &ray.direction.unit());
        let p = ray.at(t);
        // println!("Intersect at {}", t);
        if in_range(t, t_min, t_max) && self.contains(p) {
            Some(HitRecord {
                n: self.normal,
                t, p,
                front_face: true,
                material: self.material
            })
        } else { None }
    }
}

// pub struct Model {
//     tris: Vec<Tri>
// }

// impl Model {
//     fn load_obj(path: &str) -> Model {

//     }
// }
// 
// impl Hittable for Model {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let mut temp: Option<HitRecord> = None;
//         let mut closest = t_max;
//         for object in self.tris.iter() {
//             let record = object.hit(ray, t_min, closest);
//             if record.is_some() {
//                 closest = record.as_ref().unwrap().t;
//                 temp = record;
//             }
//         }
//         temp
//     }
// }

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Point, radius: f64, base_color: Color) -> Sphere {
        Sphere { center, radius, material: Material { diffuse: base_color } }
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

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd)/a;
        if !in_range(t, t_min, t_max) {
            t = (-half_b + sqrtd)/a;
            if !in_range(t, t_min, t_max) {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center).unit();
        Some(HitRecord {
            p: point,
            material: self.material,
            n: normal,
            t,
            front_face: Vec3::dot(&ray.direction, &normal) > 0.0
        })
    }
}
