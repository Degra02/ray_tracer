#![allow(non_upper_case_globals)]

use rand::Rng;

use crate::vec3::Color;

const pi: f32 = std::f64::consts::PI as f32;
const infinity: f32 = std::f32::INFINITY;

pub fn deg_to_rad(deg: f32) -> f32 {
    (deg as f64 * std::f64::consts::PI / 180.0) as f32
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0..1.0)
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color[0];
    let mut g = pixel_color[1];
    let mut b = pixel_color[2];
    
    let scale = 1.0 / samples_per_pixel as f32;
    r = f32::sqrt(r * scale);
    g = f32::sqrt(g * scale);
    b = f32::sqrt(b * scale);

    println!(
        "{} {} {}",
        (256. * clamp(r, 0.0, 0.999)) as u8,
        (256. * clamp(g, 0.0, 0.999)) as u8,
        (256. * clamp(b, 0.0, 0.999)) as u8
    )
}




