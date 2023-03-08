use std::{f64::INFINITY, fs::File};

use palette::{Srgb, Pixel};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use image::{png::PNGEncoder, ColorType};
use crate::{
    hittable::{world::World, HitRecord, Hittable, hit_world, sphere::Sphere},
    material::scatter,
    ray::Ray,
    state::State,
    vec3::{
        functions::{dot, unit_vec},
        Color, Point3,
    }, utils::random_float,
};

pub fn write_image(pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create("render.png")?;
    let encoder = PNGEncoder::new(output); 
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8));
    Ok(())
}

pub fn render(state: State) {
    let image_width = state.width.unwrap() as usize;
    let image_height = state.height as usize;
    let mut pixels = vec![0; image_height * image_width * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(image_width as usize * 3).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
        render_line(band, &state, i);
    });
    write_image(&pixels, (image_width, image_height)).expect("error writing image"); 
}

pub fn render_line(pixels: &mut [u8], state: &State, y: usize) {
    let bounds = (state.width.unwrap() as usize, state.height as usize);

    for x in 0..bounds.0 {
        let mut pixel_color = Color::new(0., 0., 0.);
        for _s in 0..state.samples_per_pixel {
            let u = (x as f64 + random_float()) / (bounds.0 - 1) as f64;
            let v = (y as f64 + random_float()) / (bounds.1 - 1) as f64;
            let r = state.camera.get_ray(u, v);
            pixel_color += ray_color(r, &state.entities_vec, state.max_depth);
        }
            let mut r = pixel_color[0];
            let mut g = pixel_color[1];
            let mut b = pixel_color[2];

            let scale = 1.0 / state.samples_per_pixel as f64;
            r = f64::sqrt(r * scale);
            g = f64::sqrt(g * scale);
            b = f64::sqrt(b * scale);

            let color = Srgb::new(
                (scale * pixel_color[0]).sqrt(),
              (scale * pixel_color[1]).sqrt(),
              (scale * pixel_color[2]).sqrt(), 
            );

            let mut pixel: [u8; 3] = color.into_format().into_raw();
            // pixel[x * 3] = pixel[0];
            // pixel[x * 3 + 1] = pixel[1];
            // pixel[x * 3 + 2] = pixel[2];
    }
}

pub fn ray_color(ray: Ray, world: &Vec<Sphere>, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let hit = hit_world(world, &ray, 0.0001, INFINITY);

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
