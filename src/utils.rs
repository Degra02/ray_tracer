#![allow(non_upper_case_globals)]

use std::{
    fs::File,
    io::{Read, Write},
};

use palette::Srgb;
use rand::Rng;

use crate::{
    hittable::sphere::Sphere,
    material::{Dielectric, Lambertian, Material, Metal},
    state::State,
    vec3::{Color, Point3},
};

const pi: f64 = std::f64::consts::PI;
const infinity: f64 = std::f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0..1.0)
}

pub fn random_float_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32, file: &mut File) {
    let mut r = pixel_color[0];
    let mut g = pixel_color[1];
    let mut b = pixel_color[2];

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);

    file.write_all(
        format!(
            "{} {} {}\n",
            (256. * clamp(r, 0.0, 0.999)) as u8,
            (256. * clamp(g, 0.0, 0.999)) as u8,
            (256. * clamp(b, 0.0, 0.999)) as u8
        )
        .as_bytes(),
    )
    .unwrap();

    // println!(
    //     "{} {} {}",
    //     (256. * clamp(r, 0.0, 0.999)) as u8,
    //     (256. * clamp(g, 0.0, 0.999)) as u8,
    //     (256. * clamp(b, 0.0, 0.999)) as u8
    // )
}
