#![allow(dead_code)]
use std::{cell::RefCell, f32::INFINITY, rc::Rc};

use hittable::{HitRecord, Hittable};
use indicatif::ProgressBar;
use material::scatter;
use ray::Ray;
use vec3::functions::{unit_vec, dot};

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    vec3::{Color, Point3, Vec3}, camera::Camera, utils::{write_color, random_float}, material::Material,
};

mod camera;
mod hittable;
mod ray;
mod utils;
mod vec3;
mod material;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: i32 = 900;
pub const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

fn main() {
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let look_from = Point3::new(-2., 2., 1.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(look_from, look_at, vup, 50.0, ASPECT_RATIO);

    // World initialization
    let material_ground = Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    let material_normal = Material::Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
    let material_dielectric = Material::Dielectric { ir: 1.5};
    let material_metal2 = Material::Metal { albedo: Color::new(0.9, 0.3, 0.4), fuzz: 0.4 };

    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_normal
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        90.,
        material_ground
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(1., 0., -1.),
        -0.5,
        material_metal2
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0., 0.9, -1.),
        0.2,
        material_metal2
    ))));

    // Render
    let pb = ProgressBar::new(HEIGHT as u64);

    println!("P3\n{WIDTH} {HEIGHT}\n255");
    for j in (0..HEIGHT).rev() {
        pb.inc(1);
        for i in 0..WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.); 
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_float()) / (WIDTH - 1) as f32;
                let v = (j as f32 + random_float()) / (HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &mut world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    pb.finish_and_clear();
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
