use std::env;

mod vec3;
mod color;
mod ray;
mod hittables;
mod utils;
mod camera;
mod ppm;

use ppm::{generate_ppm, write_ppm_file};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!("Arguments must be: out_file_name width ar_num ar_denom");
    }

    let out_file = args.remove(1);
    let width = match args[1].parse::<u32>() {
        Err(e) => panic!("Error parsing width: {}", e),
        Ok(val) => val
    };
    let ar_num: f64 = match args[2].parse::<f64>() {
        Err(e) => panic!("Error parsing ar_num: {}", e),
        Ok(val) => val
    };
    let ar_denom: f64 = match args[3].parse::<f64>() {
        Err(e) => panic!("Error parsing ar_denom: {}", e),
        Ok(val) => val
    };
    let aspect_ratio = ar_num / ar_denom;

    let file_contents = generate_ppm(width, aspect_ratio);

    write_ppm_file(out_file, file_contents)
}

