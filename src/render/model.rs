use crate::math::*;
use super::{Hittable, HitRecord};
use super::prim::*;

#[derive(Debug)]
pub struct Model {
    tris: Vec<Tri>
}

impl Model {
    pub fn load_obj(path: &str) -> Model {
        // TODO: load and use materials, multiple models
        let (models, _materials) = tobj::load_obj(path, &tobj::LoadOptions::default()).expect("Unable to load OBJ");
        let mesh = &models[0].mesh;

        let num_faces = mesh.indices.len() / 3;
        
        let mut tris = Vec::with_capacity(num_faces);

        for f in mesh.indices.chunks(3) {
            let v1_idx = 3 * f[0] as usize;
            let v2_idx = 3 * f[1] as usize;
            let v3_idx = 3 * f[2] as usize;
            let v1 = Vec3::new(mesh.positions[v1_idx].into(), mesh.positions[v1_idx + 1].into(), mesh.positions[v1_idx + 2].into());
            let v2 = Vec3::new(mesh.positions[v2_idx].into(), mesh.positions[v2_idx + 1].into(), mesh.positions[v2_idx + 2].into());
            let v3 = Vec3::new(mesh.positions[v3_idx].into(), mesh.positions[v3_idx + 1].into(), mesh.positions[v3_idx + 2].into());

            let face = Tri::new(v1, v2, v3, Vec3::cross(&(v2 - v1), &(v3 - v1)).unit(), Color::rgb(105, 72, 45));
            
            tris.push(face);
        }
        
        Model { tris }
    }

    pub fn translate(&mut self, by: Vec3) {
        for tri in self.tris.iter_mut() {
            tri.translate(by);
        }
    }
}

impl Hittable for Model {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp: Option<HitRecord> = None;
        let mut closest = t_max;
        for object in self.tris.iter() {
            let record = object.hit(ray, t_min, closest);
            if record.is_some() {
                closest = record.as_ref().unwrap().t;
                temp = record;
            }
        }
        temp
    }
}
