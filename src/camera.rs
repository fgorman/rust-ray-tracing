use crate::vec3::{Point3, Vec3, F64Multiplier};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner: Point3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + F64Multiplier(u)*self.horizontal + F64Multiplier(v)*self.vertical - self.origin
        )
    }
}