use std::f64::INFINITY;

use palette::Srgb;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    hittable::{world::World, HitRecord, Hittable, hit_world},
    material::scatter,
    ray::Ray,
    state::State,
    vec3::{
        functions::{dot, unit_vec},
        Color, Point3,
    },
};

pub fn render(state: State) {
    let image_width = state.width.unwrap();
    let image_height = state.height;
    let mut pixels = vec![0; image_height * image_width * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(image_width * 3).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
        render_line(band, &state, i);
    });
}

pub fn render_line(pixels: &mut [u8], state: &State, y: usize) {
    let bounds = (state.width.unwrap(), state.height);

    for x in 0..bounds.0 {
        let mut pixel_color = Color::new(0., 0., 0.);
        for _s in 0..bounds.1 {
            let u = (x as f64 + random_float()) / (bounds.0 - 1) as f64;
            let v = (y as f64 + random_float()) / (bounds.1 - 1) as f64;
            let r = state.camera.get_ray(u, v);
            pixel_color += ray_color(r, &mut world, state.max_depth);
            let mut r = pixel_color[0];
            let mut g = pixel_color[1];
            let mut b = pixel_color[2];

            let scale = 1.0 / samples_per_pixel as f64;
            r = f64::sqrt(r * scale);
            g = f64::sqrt(g * scale);
            b = f64::sqrt(b * scale);

            let color = Srgb::new(
                (scale * pixel_colors[0]).sqrt(),
              (scale * pixel_colors[1]).sqrt(),
              (scale * pixel_colors[2]).sqrt(), 
            );
        }
    }
}

pub fn ray_color(ray: Ray, world: &Vec<Sphere>, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let hit = hit_world(ray, 0.0001, INFINITY, &mut rec);

    if hit.is_some() {
        let hit_record = hit.unwrap();
        // let target: Point3 = rec.p + rec.normal + Vec3::random_unit_vector();
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Color = Color::default();
        if scatter(rec.material, ray, rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0., 0., 0.);
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
