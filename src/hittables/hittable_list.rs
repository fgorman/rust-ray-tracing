use std::rc::Rc;

use crate::ray::Ray;

use super::hittable::{Hittable, HitRecord};

struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn new(object: Rc<dyn Hittable>) -> Self{
        Self {
            objects: vec![object]
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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