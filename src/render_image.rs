use std::{rc::Rc, io::{self, Write}};

use image::{RgbImage, ImageBuffer, Rgb};

use crate::{
    hittables::{hittable_list::HittableList,sphere::Sphere},
    vec3::{Point3},
    camera::Camera,
    ray::Ray,
    utils::random_double,
    rgb_wrapper::RgbWrapper,
    color::{write_color, ray_color}
};

const SAMPLES_PER_PIXEL: u32 = 100;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const MAX_DEPTH: u32 = 50;
const GAMMA: f64 = 2.0;

pub fn render_image(out_file: String, image_width: u32, aspect_ratio: f64) {
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    let mut world: HittableList = HittableList::new_empty();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam: Camera = Camera::new(aspect_ratio, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    for j in (0..image_height).rev() {
        print!("\rScanlines remanining: {}", j);
        if let Err(e) = io::stdout().flush() {
            panic!("Error with flushing stdout: {}", e);
        }
        for i in 0..image_width {
            let mut pixel_color: RgbWrapper = RgbWrapper(Rgb::from([0.0, 0.0, 0.0]));
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v: f64 = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            // image buffer starts from the bottom left, so we have to convert from top left to bottom right
            let pixel_x = (image_width - 1) - i;
            let pixel_y = (image_height - 1) - j; 
            image.put_pixel(pixel_x, pixel_y, write_color(pixel_color.0, SAMPLES_PER_PIXEL, GAMMA));
        }
    }

    match image.save(&out_file) {
        Err(ex) => panic!("Error with saving image: {}", ex),
        Ok(_) => println!("\nImage saved to file: {}", out_file)
    }
}
