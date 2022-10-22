use crate::hittables::hittable::{HitRecord, Hittable};
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Self {
            center: Point3::new_empty(),
            radius: 0.0
        }
    }

    #[allow(dead_code)]
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit (self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = oc.dot(r.direction());
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }
}