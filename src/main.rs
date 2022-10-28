mod arguments;
mod vec3;
mod color;
mod ray;
mod hittables;
mod utils;
mod camera;
mod ppm;
mod materials;

use arguments::{Args, parse_command_line_args};
use ppm::{generate_ppm, write_ppm_file};

fn main() {

    let args: Args = parse_command_line_args();
    
    let aspect_ratio: f64 = args.numerator_ar / args.denominator_ar;

    let file_contents = generate_ppm(args.image_width, aspect_ratio);

    write_ppm_file(args.out_file, file_contents)
}

