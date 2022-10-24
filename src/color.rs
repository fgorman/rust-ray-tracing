use crate::{vec3::Color, utils::clamp};

const COLOR_MULTIPLIER: f64 = 256.0;
const CLAMP_MIN: f64 = 0.0;
const CLAMP_MAX: f64 = 0.999;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) -> String {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    format!(
        "{} {} {}\n",
        (COLOR_MULTIPLIER*clamp(r, CLAMP_MIN, CLAMP_MAX)) as i32,
        (COLOR_MULTIPLIER*clamp(g, CLAMP_MIN, CLAMP_MAX)) as i32,
        (COLOR_MULTIPLIER*clamp(b, CLAMP_MIN, CLAMP_MAX)) as i32
    )
}