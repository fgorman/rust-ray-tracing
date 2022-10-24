use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_empty() -> Self {
        HitRecord { 
            p: Point3::new_empty(),
            normal: Vec3::new_empty(),
            t: 0.0,
            front_face: false
         }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit (&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}