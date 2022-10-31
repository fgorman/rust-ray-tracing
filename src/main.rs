mod arguments;
mod vec3;
mod color;
mod ray;
mod hittables;
mod utils;
mod camera;
mod materials;
mod render_image;
mod rgb_wrapper;

use arguments::{Args, parse_command_line_args};
use render_image::render_image;

fn main() {
    let args: Args = parse_command_line_args();
    
    let aspect_ratio: f64 = args.numerator_ar / args.denominator_ar;

    render_image(args.out_file, args.image_width, aspect_ratio, args.multithread)
}
