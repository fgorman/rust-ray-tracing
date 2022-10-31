use std::sync::Arc;

use crate::ray::Ray;

use super::hittable::{Hittable, HitRecord};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new_empty() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn new(object: Arc<dyn Hittable + Send + Sync>) -> Self{
        Self {
            objects: vec![object]
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit (&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new_empty();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}