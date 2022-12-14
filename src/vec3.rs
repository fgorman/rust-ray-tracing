use std::ops::{Neg, AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};
use std::fmt::Display;

use crate::utils::{random_double, random_double_from_range};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new_empty() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    #[allow(dead_code)]
    pub fn new_random() -> Self {
        Self {
            e: [random_double(), random_double(), random_double()]
        }
    }

    pub fn new_random_in_range(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_double_from_range(min, max),
                random_double_from_range(min, max),
                random_double_from_range(min, max)
            ]
        }
    }

    #[allow(dead_code)]
    pub fn x(self) -> f64 { self.e[0] }
    pub fn y(self) -> f64 { self.e[1] }
    #[allow(dead_code)]
    pub fn z(self) -> f64 { self.e[2] }

    pub fn length_squared(self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.e[0]*other.e[0] + self.e[1]*other.e[1] + self.e[2]*other.e[2]
    }

    #[allow(dead_code)]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0]
            ]
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], self.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0/other;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        let e0 = self.e[0] + _rhs.e[0];
        let e1 = self.e[1] + _rhs.e[1];
        let e2 = self.e[2] + _rhs.e[2];
        
        Self {
            e: [e0, e1, e2]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        let e0 = self.e[0] - _rhs.e[0];
        let e1 = self.e[1] - _rhs.e[1];
        let e2 = self.e[2] - _rhs.e[2];
        
        Self {
            e: [e0, e1, e2]
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        let e0 = self.e[0] * _rhs.e[0];
        let e1 = self.e[1] * _rhs.e[1];
        let e2 = self.e[2] * _rhs.e[2];
        
        Self {
            e: [e0, e1, e2]
        }
    }
}

pub struct Vec3Multiplier(pub Vec3);

impl Mul<f64> for Vec3Multiplier {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Self::Output {
        let e0 = self.0.e[0] * _rhs;
        let e1 = self.0.e[1] * _rhs;
        let e2 = self.0.e[2] * _rhs;
        
        Self::Output {
            e: [e0, e1, e2]
        }
    }
}

pub struct F64Multiplier(pub f64);

impl Mul<Vec3> for F64Multiplier {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        let e0: f64 = self.0 * _rhs.e[0];
        let e1: f64 = self.0 * _rhs.e[1];
        let e2: f64 = self.0 * _rhs.e[2];

        Vec3 {
            e: [e0, e1, e2]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        F64Multiplier(1.0/_rhs) * self
    }
}

//////////////////////////////////////////////////////
/// Diffuse Methods
/// 
/// Allow dead_code so we can swap in and out
/// whenever we want
/////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new_random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere: Vec3 = random_in_unit_sphere();
    if normal.dot(in_unit_sphere) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}