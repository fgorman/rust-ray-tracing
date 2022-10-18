use std::{env, fs::File, io::{self, Write}};

mod vec3;
use vec3::{Vec3, Color};

mod color;
use color::write_color;

fn generate_ppm(width: u32, height: u32) -> String {
    let mut file_contents: String = String::new();

    // PPM metadata
    file_contents += "P3\n";
    file_contents += &format!("{} {}\n", width, height);
    file_contents += "255\n";

    for j in (0..height).rev() {
        print!("\rScanlines remanining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..width {
            let c: Color = Vec3::new(
                (i as f32 / (width as f32 - 1.0)) as f32,
                (j as f32 / (height as f32 - 1.0)) as f32,
                0.25
            );
            file_contents += &write_color(c)
        }
    }

    println!("\nDone");

    file_contents
}

fn write_ppm_file(file_name: String, file_contents: String) {
    let mut file = match File::create(file_name) {
        Err(e) => panic!("{}", e),
        Ok(file) => file
    };

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("Wrote ppm file")
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Arguments must be: out_file_name width height");
    }

    let out_file = args.remove(1);
    let width = args[1].parse::<u32>().unwrap();
    let height = args[2].parse::<u32>().unwrap();

    let file_contents = generate_ppm(height, width);

    write_ppm_file(out_file, file_contents)
}

