#![allow(dead_code)]
use indicatif::ProgressBar;
use ray::Ray;
use vec3::utils::{unit_vec, dot};

use crate::vec3::{utils::write_color, Color, Point3, Vec3};

mod ray;
mod vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: i32 = 400;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;


fn main() {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_lenght);

    // Render

    let pb = ProgressBar::new(HEIGHT as u64);
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..HEIGHT).rev() {
        pb.inc(1);
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = j as f32 / (HEIGHT - 1) as f32;
            let r = Ray::new(
                origin.clone(),
                lower_left_corner + u*horizontal + v*vertical - origin.clone(),
            );

            let pixel_color = ray_color(r);

            write_color(pixel_color);
        }
    }
    pb.finish_with_message("Done");
}

pub fn ray_color(ray: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0., 0., -1.), 0.6, ray);
    if  t > 0.0 {
        let n = unit_vec(ray.at(t) - Vec3::new(0., 0., -1.));
        return 0.5 * Color::new(n[0] + 1., n[1] + 1., n[2] + 1.); 
    }
    let unit_direction = unit_vec(ray.dir());
    t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


pub fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.dir().norm_squared(); 
    let half_b = dot(oc, ray.dir());
    let c = oc.norm_squared() - radius * radius;    
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.
    } else {
        (-half_b - f32::sqrt(discriminant) ) / a
    }
}

#[test]
fn test_vecs() {
    let mut v1 = Vec3::new(1., 2., 0.0);
    let mut v2 = Vec3::new(1., 1., 0.0);

    println!("{}", v1*v2);
    println!("{}", v1 + v2);
    println!("{}", v1 - v2);
    println!("{}", dot(v1, v2));
    println!("{}", -v2);
}





