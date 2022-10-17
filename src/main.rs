use std::{env, fs::File, io::{Write, self}};

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
            let r: f32 = i as f32 / (width - 1) as f32;
            let g: f32 = j as f32 / (height - 1) as f32;
            let b: f32 = 0.25;

            let ir: u8 = (255.99 * r) as u8;
            let ig: u8 = (255.99 * g) as u8;
            let ib: u8 = (255.99 * b) as u8;

            file_contents += &format!("{} {} {}\n", ir, ig, ib);
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
