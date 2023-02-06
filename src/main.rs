#![allow(dead_code)]
use indicatif::ProgressBar;
use ray::Ray;
use vec3::utils::{unit_vec, dot};

use crate::vec3::{utils::write_color, Color, Point3, Vec3};

mod ray;
mod vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: i32 = 600;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;


fn main() {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_lenght);

    // Render

    let pb = ProgressBar::new(HEIGHT as u64);
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..HEIGHT - 1).rev() {
        pb.inc(1);
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = j as f32 / (HEIGHT - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u*horizontal + v*vertical - origin,
            );

            let pixel_color = ray_color(r);

            write_color(pixel_color);
        }
    }
    pb.finish_with_message("Done");
}

pub fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0., 0., -1.0), 0.05, ray) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = unit_vec(ray.dir());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


pub fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> bool {
    let oc = ray.origin() - center;
    let a = dot(ray.dir(), ray.dir()); 
    let b = 2. * dot(oc, ray.dir());
    let c = dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.*a*c;

    discriminant > 0.
}







