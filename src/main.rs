#![allow(dead_code)]
use std::{cell::RefCell, f32::INFINITY, rc::Rc};

use hittable::{HitRecord, Hittable};
use indicatif::ProgressBar;
use ray::Ray;
use vec3::utils::{dot, unit_vec};

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    vec3::{Color, Point3, Vec3}, camera::{ASPECT_RATIO, HEIGHT, WIDTH, Camera}, utils::{write_color, random_float},
};

mod camera;
mod hittable;
mod ray;
mod utils;
mod vec3;

fn main() {
    let samples_per_pixel = 100;

    // Camera
    let camera = Camera::new();

    // World initialization
    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
    ))));

    // Render
    let pb = ProgressBar::new(HEIGHT as u64);
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..HEIGHT).rev() {
        pb.inc(1);
        for i in 0..WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.); 
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_float()) / (WIDTH - 1) as f32;
                let v = (j as f32 + random_float()) / (HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &mut world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    pb.finish_with_message("Done");
}

pub fn ray_color(ray: Ray, world: &mut dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0., INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }
    let unit_direction = unit_vec(ray.dir());
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.dir().norm_squared();
    let half_b = dot(oc, ray.dir());
    let c = oc.norm_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.
    } else {
        (-half_b - f32::sqrt(discriminant)) / a
    }
}
