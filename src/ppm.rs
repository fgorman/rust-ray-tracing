use std::{rc::Rc, io::{self, Write}, fs::File};

use crate::{
    hittables::{hittable_list::HittableList, sphere::Sphere, hittable::{Hittable, HitRecord}},
    vec3::{Point3, Vec3, F64Multiplier, Color},
    ray::Ray,
    color::write_color,
    camera::Camera,
    utils::random_double
};

const SAMPLES_PER_PIXEL: u32 = 100;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::new_empty();
    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return F64Multiplier(0.5) * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    F64Multiplier(1.0-t)*Color::new(1.0, 1.0, 1.0) + F64Multiplier(t) * Color::new(0.5, 0.7, 1.0)
}

pub fn generate_ppm(width: u32, aspect_ratio: f64) -> String {
    // Image
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    // World
    let mut world: HittableList = HittableList::new_empty();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam: Camera = Camera::new(aspect_ratio, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    let mut file_contents: String = String::new();

    // PPM metadata
    file_contents += "P3\n";
    file_contents += &format!("{} {}\n", width, height);
    file_contents += "255\n";

    for j in (0..height).rev() {
        print!("\rScanlines remanining: {}", j);
        if let Err(e) = io::stdout().flush() {
            panic!("Error with flushing stdout: {}", e);
        }
        for i in 0..width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_double()) / (width - 1) as f64;
                let v: f64 = (j as f64 + random_double()) / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            file_contents += &write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    println!("\nDone");

    file_contents
}

pub fn write_ppm_file(file_name: String, file_contents: String) {
    let mut file = match File::create(file_name) {
        Err(e) => panic!("{}", e),
        Ok(file) => file
    };

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("Wrote ppm file")
    }
}