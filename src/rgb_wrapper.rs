use std::ops::{Add, Mul, AddAssign};

use image::Rgb;

pub struct RgbWrapper(pub Rgb<f64>);

impl Add for RgbWrapper {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        let lhs_rgb = self.0.0;
        let rhs_rgb = _rhs.0.0;

        Self(Rgb::from([
            lhs_rgb[0] + rhs_rgb[0],
            lhs_rgb[1] + rhs_rgb[1],
            lhs_rgb[2] + rhs_rgb[2]
        ]))
    }
}

impl Mul<f64> for RgbWrapper {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        let lhs_rgb = self.0.0;

        Self(Rgb::from([
            lhs_rgb[0] * _rhs,
            lhs_rgb[1] * _rhs,
            lhs_rgb[2] * _rhs
        ]))
    }
}

impl AddAssign for RgbWrapper {
    fn add_assign(&mut self, rhs: Self) {
        self.0.0[0] += rhs.0.0[0];
        self.0.0[1] += rhs.0.0[1];
        self.0.0[2] += rhs.0.0[2]
    }
}