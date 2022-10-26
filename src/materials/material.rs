use crate::ray::Ray;
use crate::hittables::hittable::HitRecord;
use crate::vec3::Color;

pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}