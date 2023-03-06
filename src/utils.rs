#![allow(non_upper_case_globals)]

use std::{
    fs::File,
    io::{Read, Write},
};

use rand::Rng;

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::Material,
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

pub fn gen_random_spheres(world: &mut HittableList, n: i32) {
    for x in -n..n {
        for z in -n..n {
            let albedo = Color::new(random_float(), random_float(), random_float());
            let center = Point3::new(x as f64 + random_float(), 0.2, z as f64 + random_float());
            let material: Material;
            let mat = random_float();
            match mat {
                _ if mat < -0.25 => material = Material::Lambertian { albedo },

                _ if mat < 0.25 => material = Material::Metal { albedo, fuzz: mat },

                _ => {
                    material = Material::Dielectric {
                        ir: random_float_range(-0.7, 0.7),
                    }
                }
            }
            let sphere = Sphere::new(center, 0.2, material);
            world.add(sphere);
        }
    }
}

pub fn json_parser(world: &mut HittableList) -> State {
    let mut file = File::open("state.json").unwrap();
    let mut to_parse = String::new();
    let _res = file.read_to_string(&mut to_parse);
    // println!("{}", to_parse);

    let state_deser: State = serde_json::from_str(&to_parse).unwrap();

    world.add_vec(state_deser.entities_vec.clone());

    state_deser
}
