use std::{env, fs::File, io::{self, Write}};

mod vec3;
use vec3::{Color, Vec3, Point3, F64Multiplier};

mod color;
use color::write_color;

mod ray;
use ray::Ray;

const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(ray: Ray) -> Color {
    let unit_vector: Vec3 = ray.direction().unit_vector();
    let t: f64 = 0.5*(unit_vector.y() + 1.0);
    F64Multiplier(1.0-t)*Color::new(1.0, 1.0, 1.0) + F64Multiplier(t)*Color::new(0.5, 0.7, 1.0)
}

fn generate_ppm(width: u32, aspect_ratio: f64) -> String {
    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let viewport_width: f64 = aspect_ratio * VIEWPORT_HEIGHT;

    let origin: Point3 = Point3::new_empty();
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut file_contents: String = String::new();

    // PPM metadata
    file_contents += "P3\n";
    file_contents += &format!("{} {}\n", width, height);
    file_contents += "255\n";

    for j in (0..height).rev() {
        print!("\rScanlines remanining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..width {
            let u: f64 = i as f64 / (width - 1) as f64;
            let v: f64 = j as f64 / (height - 1) as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + F64Multiplier(u)*horizontal + F64Multiplier(v)*vertical - origin);
            let c: Color = ray_color(r);
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

    if args.len() < 5 {
        panic!("Arguments must be: out_file_name width ar_num ar_denom");
    }

    let out_file = args.remove(1);
    let width = args[1].parse::<u32>().unwrap();
    let ar_num: f64 = args[2].parse::<f64>().unwrap();
    let ar_denom: f64 = args[3].parse::<f64>().unwrap();
    let aspect_ratio = ar_num / ar_denom;

    let file_contents = generate_ppm(width, aspect_ratio);

    write_ppm_file(out_file, file_contents)
}

