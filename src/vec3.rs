#![allow(dead_code)]

use std::ops::{Neg, AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new_empty() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f32 { self.e[0] }
    pub fn y(self) -> f32 { self.e[1] }
    pub fn z(self) -> f32 { self.e[2] }

    pub fn length_squared(self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn to_string(self) -> String {
        format!("{} {} {}", self.e[0], self.e[1], self.e[2])
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.e[0]*other.e[0] + self.e[1]+other.e[1] + self.e[2]*other.e[2]
    }

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

pub type Color = Vec3;
pub type Point3 = Vec3;

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

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
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

pub struct Vec3Multiplier(Vec3);

impl Mul<f32> for Vec3Multiplier {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Self::Output {
        let e0 = self.0.e[0] * _rhs;
        let e1 = self.0.e[1] * _rhs;
        let e2 = self.0.e[2] * _rhs;
        
        Self::Output {
            e: [e0, e1, e2]
        }
    }
}

pub struct F32Multiplier(f32);

impl Mul<Vec3> for F32Multiplier {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        let e0: f32 = self.0 * _rhs.e[0];
        let e1: f32 = self.0 * _rhs.e[1];
        let e2: f32 = self.0 * _rhs.e[2];

        Vec3 {
            e: [e0, e1, e2]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        F32Multiplier(1.0/_rhs) * self
    }
}