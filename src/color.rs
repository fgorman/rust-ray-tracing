use image::Rgb;

use crate::{
    utils::clamp,
    hittables::hittable::{Hittable, HitRecord},
    rgb_wrapper::RgbWrapper,
    ray::Ray,
    vec3::{Point3, random_unit_vector, Vec3}};

const COLOR_MULTIPLIER: f64 = 256.0;
const CLAMP_MIN: f64 = 0.0;
const CLAMP_MAX: f64 = 0.999;

pub fn write_color(pixel_color: Rgb<f64>, samples_per_pixel: u32, gamma: f64) -> Rgb<u8> {
    let mut r: f64 = pixel_color.0[0];
    let mut g: f64 = pixel_color.0[1];
    let mut b: f64 = pixel_color.0[2];

    // Scale and gamma correct each pixel
    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    let gamma_correct: f64 = 1.0 / gamma;
    r = (scale * r).powf(gamma_correct);
    g = (scale * g).powf(gamma_correct);
    b = (scale * b).powf(gamma_correct);

    Rgb::from([
        (COLOR_MULTIPLIER*clamp(r, CLAMP_MIN, CLAMP_MAX)) as u8,
        (COLOR_MULTIPLIER*clamp(g, CLAMP_MIN, CLAMP_MAX)) as u8,
        (COLOR_MULTIPLIER*clamp(b, CLAMP_MIN, CLAMP_MAX)) as u8
    ])
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> RgbWrapper {
    let mut rec: HitRecord = HitRecord::new_empty();

    if depth == 0 {
        return RgbWrapper(Rgb::from([0.0, 0.0, 0.0]));
    }

    if world.hit(ray, 0.0001, f64::INFINITY, &mut rec) {
        let target: Point3 = rec.p + rec.normal + random_unit_vector();
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth-1) * 0.5;
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    RgbWrapper(
        (RgbWrapper(Rgb::from([1.0, 1.0, 1.0]))*(1.0-t) + RgbWrapper(Rgb::from([0.5, 0.7, 1.0]))*t).0
    )
}