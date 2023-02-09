#![allow(dead_code)]

use fast_inv_sqrt::InvSqrt64;

use crate::utils::{random_float, random_float_range};

use super::{Vec3, Point3};
use std::fmt::Display;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn norm(&self) -> f64 {
        1.0 / (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).inv_sqrt64() 
        // f64::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
    }

    pub fn norm_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(random_float_range(min, max), random_float_range(min, max), random_float_range(min, max))
    }
    
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1., 1.);
            if p.norm_squared() >= 1. {
                continue;
            } else {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        unit_vec(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    )
}

pub fn unit_vec(vec: Vec3) -> Vec3 {
    // vec / vec.norm()
    vec * (vec[0]*vec[0] + vec[1]*vec[1] + vec[2]*vec[2]).inv_sqrt64()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_eta: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_eta * (uv + cos_theta*n);
    let r_out_parallel = -f64::sqrt((1.0 - r_out_perp.norm_squared()).abs()) * n;

    r_out_perp + r_out_parallel
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
        )
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self[0], -self[1], -self[2])
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}
