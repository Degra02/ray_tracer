use std::{f64::INFINITY, fs::File, time::Instant, borrow::Borrow};

use crate::{
    hittable::{hit_world, sphere::Sphere},
    material::Scatterable,
    ray::Ray,
    state::State,
    utils::{clamp, random_float},
    vec3::{
        functions::{dot}, Point3,
    },
};
use image::{png::PNGEncoder, ColorType};
use indicatif::{ProgressBar, ProgressStyle};
use palette::Pixel;
use palette::Srgb;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn write_image(pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create("render.png")?;
    let encoder = PNGEncoder::new(output);
    let _err = encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8));
    Ok(())
}

pub fn render(state: State) {
    let image_width = state.width.unwrap() as usize;
    let image_height = state.height as usize;
    let mut pixels = vec![0; image_height * image_width * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(image_width * 3).enumerate().collect();

    let pb = ProgressBar::new(state.height as u64);
    let sty =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix} {bar:40.cyan/blue} {pos:>7}/{len:7} \n {msg}")
            .unwrap();
    pb.set_style(sty);

    let start = Instant::now();
    bands.into_par_iter().for_each(|(i, band)| {
        pb.inc(1);
        render_line(band, &state, i);
    });
    pb.finish_with_message("[Render Complete!]");
    println!("Time elapsed: {}ms", start.elapsed().as_millis());

    write_image(&pixels, (image_width, image_height)).expect("error writing image");
}

pub fn render_line(pixels: &mut [u8], state: &State, y: usize) {
    let bounds = (state.width.unwrap() as usize, state.height as usize);

    for x in 0..bounds.0 {
        let mut pixel_colors: Vec<f32> = vec![0.0; 3];
        for _s in 0..state.samples_per_pixel {
            let u = (x as f64 + random_float()) / (bounds.0 - 1) as f64;
            let v = (bounds.1 as f64 - (y as f64 + random_float())) / (bounds.1 - 1) as f64;
            let r = state.camera.get_ray(u, v);
            let c = ray_color(r, &state.entities_vec, state.lights.as_ref(), state.max_depth, state.max_depth);    
            pixel_colors[0] += c.red;
            pixel_colors[1] += c.green;
            pixel_colors[2] += c.blue
        }
        let scale = 1.0 / state.samples_per_pixel as f32;

        let color = Srgb::new(
            (scale * pixel_colors[0]).sqrt(),
            (scale * pixel_colors[1]).sqrt(),
            (scale * pixel_colors[2]).sqrt(),
        );

        let pixel: [u8; 3] = color.into_format().into_raw();
        pixels[x * 3] = pixel[0];
        pixels[x * 3 + 1] = pixel[1];
        pixels[x * 3 + 2] = pixel[2];
    }
}

pub fn ray_color(ray: Ray, world: &Vec<Sphere>, lights_opt: Option<&Vec<Sphere>>, max_depth: i32, depth: i32) -> Srgb {
    if depth <= 0 {
        return Srgb::new(0., 0., 0.);
    }

    let hit = hit_world(world, &ray, 0.0001, INFINITY);

    match hit {
        Some(hit_record) => {
            let mut light_red: f64 = 0.0;
            let mut light_green = 0.0;
            let mut light_blue = 0.0;
            let prob = 0.1;
            let scattered = hit_record.material.scatter(&ray, &hit_record);

            match scattered {
                Some((scattered_ray, albedo)) => {
                    match lights_opt {
                        Some(lights) => {
                        if lights.len() > 0 && depth > (max_depth - 2) && rand::thread_rng().gen::<f64>() > (1.0 - lights.len() as f64 * prob){
                            for light in lights {
                                let light_ray = Ray::new(hit_record.p, light.center - hit_record.p);
                                let target_color = ray_color(light_ray, world, Some(lights), 2, 1);
                                light_red += (albedo.red * target_color.red) as f64;
                                light_green += (albedo.green * target_color.green) as f64;
                                light_blue += (albedo.blue * target_color.blue) as f64; 
                            }
                            light_red /= lights.len() as f64;
                            light_green /= lights.len() as f64;
                            light_blue /= lights.len() as f64;
                        }
                        },
                        None => {},
                    } 

                    match scattered_ray {        
                    Some(sr) => {
                        let target_color = ray_color(sr, world, lights_opt, max_depth, depth - 1);
                        Srgb::new(
                            clamp(
                                (light_red as f32 + albedo.red * target_color.red) as f64,
                                0.,
                                1.,
                            ) as f32,
                            clamp(
                                (light_green as f32 + albedo.green * target_color.green) as f64,
                                0.,
                                1.,
                            ) as f32,
                            clamp(
                                (light_blue as f32 + albedo.blue * target_color.blue) as f64,
                                0.,
                                1.,
                            ) as f32,
                        )
                }
                    None => albedo,
                }
            }
                None => Srgb::new(0., 0., 0.),
            }
        }
        None => {
            let t = clamp(0.5 * (ray.dir().unit_vec().y() + 1.0), 0., 1.);
            let _u: f64 = clamp(0.5 * (ray.dir().unit_vec().x() + 1.0), 0., 1.);
            Srgb::new(
                ((1.0 - t) * 1.0 + t * 0.5) as f32,
                ((1.0 - t) * 1.0 + t * 0.7) as f32,
                ((1.0 - t) * 1.0 + t * 1.0) as f32,
            )
        }
    }
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
