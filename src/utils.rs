use rand::distributions::{Uniform, Distribution};

pub fn random_double() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn random_double_from_range(start: f64, end: f64) -> f64 {
    let between = Uniform::from(start..end);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}