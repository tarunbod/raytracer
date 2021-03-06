use crate::math::*;
use crate::render::{Material, Hittable, HitRecord};

#[derive(Debug)]
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

    pub fn calc(p1: Point, p2: Point, p3: Point) -> Tri {
        let normal = Vec3::cross(&(p2 - p1), &(p3 - p1)).unit();
        Self::new(p1, p2, p3, normal, normal)
    }

    pub fn translate(&mut self, by: Vec3) {
        self.p1 += by;
        self.p2 += by;
        self.p3 += by;
    }
}

impl Hittable for Tri {
    // Möller-Trumbore algorithm for ray-triangle intersection
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let v0v1 = self.p2 - self.p1;
        let v0v2 = self.p3 - self.p1;
        let pvec = Vec3::cross(&ray.direction, &v0v2);
        let det = Vec3::dot(&v0v1, &pvec);
        if det < 0.001 { return None; }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.p1;
        let u = Vec3::dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 { return None; }

        let qvec = Vec3::cross(&tvec, &v0v1);
        let v = Vec3::dot(&ray.direction, &qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 { return None; }

        let t = Vec3::dot(&v0v2, &qvec) * inv_det;
        if in_range(t, t_min, t_max) {
            Some(HitRecord {
                n: self.normal,
                t,
                p: ray.at(t),
                front_face: true,
                material: self.material
            })
        } else { None }
    }
}

#[derive(Debug)]
pub struct Rect {
    tri1: Tri,
    tri2: Tri
}

impl Rect {
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point, normal: Vec3, color: Color) -> Rect {
        Rect {
            tri1: Tri::new(p1, p2, p3, normal, color),
            tri2: Tri::new(p2, p3, p4, normal, color)
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.tri1.hit(ray, t_min, t_max).or(self.tri2.hit(ray, t_min, t_max))
    }
}

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
