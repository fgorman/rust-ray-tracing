use std::rc::Rc;

use crate::ray::Ray;
use crate::hittables::hittable::HitRecord;
use crate::rgb_wrapper::RgbWrapper;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RgbWrapper, scattered: &mut Ray) -> bool;
}

pub struct Material {
    pub mat_type: Rc<dyn Scatter>,
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RgbWrapper, scattered: &mut Ray) -> bool {
        self.mat_type.scatter(r_in, rec, attenuation, scattered)
    }
}