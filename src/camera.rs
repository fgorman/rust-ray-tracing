use crate::vec3::{Point3, Vec3, F64Multiplier};
use crate::ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(VIEWPORT_HEIGHT, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_WIDTH, 0.0);
        let lower_left_corner: Point3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

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