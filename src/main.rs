#![allow(dead_code)]
use std::{cell::RefCell, f64::INFINITY, rc::Rc, fs::File, io::Write};

use hittable::{HitRecord, Hittable};
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use material::scatter;
use ray::Ray;
use utils::json_parser;
use vec3::functions::{unit_vec, dot};

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    vec3::{Color, Point3, Vec3}, camera::Camera, utils::{write_color, random_float, gen_random_spheres}, material::Material,
};

mod camera;
mod hittable;
mod ray;
mod utils;
mod vec3;
mod material;

mod tests;

pub const ASPECT_RATIO: f64 = 1.0; //16.0 / 9.0;
pub const WIDTH: i32 = 1024;
pub const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO) as i32;
pub const FRAMES: u32 = 1;

fn main() {
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let look_from = Point3::new(0., 1., 1.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(look_from, look_at, vup, 50.0, ASPECT_RATIO);

    // World initialization
    let mut world = HittableList::default();
    json_parser(&mut world); 

    // Render

    let pb = ProgressBar::new(HEIGHT as u64); 
    let sty = ProgressStyle::with_template(
        "[{msg}] {bar:40.cyan/blue} {pos:>7}/{len:7}",
    ).unwrap();

    pb.set_style(sty);

    for frame in 0..FRAMES {
        pb.set_message(format!("Frame {}/{}", frame + 1, FRAMES));
        let mut file = File::create(format!("./data/{:04}.ppm", frame)).unwrap(); 
        file.write_all(format!("P3\n{WIDTH} {HEIGHT}\n255\n").as_bytes()).unwrap();
        
        for j in (0..HEIGHT).rev() {
            pb.inc(1);
            for i in 0..WIDTH {
                let mut pixel_color = Color::new(0., 0., 0.); 
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random_float()) / (WIDTH - 1) as f64;
                    let v = (j as f64 + random_float()) / (HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(r, &mut world, max_depth);
                }
                write_color(pixel_color, samples_per_pixel, &mut file);
            }
        }
        pb.reset();

    }
    pb.finish_with_message("Rendering complete!");
}

pub fn ray_color(ray: Ray, world: &mut dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
       return Color::new(0., 0., 0.) 
    }

    if world.hit(ray, 0.0001, INFINITY, &mut rec) {
        // let target: Point3 = rec.p + rec.normal + Vec3::random_unit_vector();
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Color = Color::default();
        if scatter(rec.material, ray, rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth-1);
        }
        return Color::new(0., 0., 0.)
    }
    let unit_direction = unit_vec(ray.dir());
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.7, 0.7, 1.0)
}

pub fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.dir().norm_squared();
    let half_b = dot(oc, ray.dir());
    let c = oc.norm_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}
