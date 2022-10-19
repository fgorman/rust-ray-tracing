use crate::vec3::Color;

const COLOR_MULTIPLIER: f64 = 255.99;

pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}\n",
        (COLOR_MULTIPLIER*pixel_color.x()) as i32,
        (COLOR_MULTIPLIER*pixel_color.y()) as i32,
        (COLOR_MULTIPLIER*pixel_color.z()) as i32
    )
}