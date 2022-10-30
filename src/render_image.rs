use std::{io::{self, Write}, thread::{available_parallelism, JoinHandle, self}, sync::{Arc, Mutex}};

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

pub fn render_image(out_file: String, image_width: u32, aspect_ratio: f64, mt: bool) {
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    let mut world: HittableList = HittableList::new_empty();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam: Camera = Camera::new(aspect_ratio, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    if mt {
        multithreaded_render(&out_file, &mut image, &world, &cam);
    } else {
        single_threaded_render(&out_file, &mut image, &world, &cam);
    }
}

fn single_threaded_render(out_file: &String, image: &mut RgbImage, world: &HittableList, cam: &Camera) {
    let image_width = image.width();
    let image_height = image.height();

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
                pixel_color += ray_color(&r, world, MAX_DEPTH);
            }
            // image buffer starts from the bottom left, so we have to convert
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

fn multithreaded_render(out_file: &String, image: &mut RgbImage, world: &HittableList, cam: &Camera) {
    let num_threads = get_num_threads();

    let image_width = image.width();
    let image_height = image.height();

    let local_image = image.clone();

    let thread_chunks = determine_thread_chunks(num_threads, image_width, image_height);

    let mut threads: Vec<JoinHandle<i32>> = Vec::new();

    let image_mutex = Arc::new(Mutex::new(local_image));

    for thread_num in 0..thread_chunks.len() {
        let chunk = thread_chunks.get(thread_num as usize).unwrap();
        let x_start = chunk.0;
        let x_end = chunk.1;
        let y_start = chunk.2;
        let y_end = chunk.3;

        let thread_world = world.clone();
        let thread_cam = cam.clone();

        let thread_image_mutex = Arc::clone(&image_mutex);

        let thread = thread::spawn(move || {
            for j in (y_start..y_end).rev() {
                for i in x_start..x_end {
                    let mut pixel_color: RgbWrapper = RgbWrapper(Rgb::from([0.0, 0.0, 0.0]));
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                        let v: f64 = (j as f64 + random_double()) / (image_height - 1) as f64;
                        let r: Ray = thread_cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &thread_world, MAX_DEPTH);
                    }
                    // image buffer starts from the bottom left, so we have to convert
                    let pixel_x = (image_width - 1) - i;
                    let pixel_y = (image_height - 1) - j;

                    let mut image_changer = match thread_image_mutex.lock() {
                        Err(ex) => panic!("Error locking mutex for image: {:?}", ex),
                        Ok(mutex) => mutex,
                    };

                    image_changer.put_pixel(pixel_x, pixel_y, write_color(pixel_color.0, SAMPLES_PER_PIXEL, GAMMA));
                }
            }
            0
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let image_to_save = match image_mutex.lock() {
        Err(ex) => panic!("Error getting image lock: {:?}", ex),
        Ok(image_lock) => image_lock,
    };

    match image_to_save.save(&out_file) {
        Err(ex) => panic!("Error with saving image: {}", ex),
        Ok(_) => println!("\nImage saved to file: {}", out_file)
    }
}

fn get_num_threads() -> u32 {
    match available_parallelism() {
        Ok(n) => n.get() as u32,
        Err(ex) => panic!("Error getting available parallelism: {:?}", ex),
    }
}

// Chunks are a tuple of (x_start, x_end, y_start, y_end)
fn determine_thread_chunks(num_threads: u32, image_width: u32, image_height: u32) -> Vec<(u32, u32, u32, u32)> {
    let mut chunks: Vec<(u32, u32, u32, u32)> = Vec::new();

    let x_steps = (image_width as f64 / num_threads as f64).ceil() as u32;
    let y_steps = (image_height as f64 / num_threads as f64).ceil() as u32;

    for j in (0..image_height).step_by(y_steps as usize) {
        let y_start = j;
        let y_end = if j + y_steps > image_height { image_height } else { j + y_steps };
        for i in (0..image_width).step_by(x_steps as usize) {
            let x_start = i;
            let x_end = if i + x_steps > image_width { image_width } else { i + x_steps };
            chunks.push((x_start, x_end, y_start, y_end));
        }
    }

    chunks
}