#![allow(dead_code)]
use std::{f64::INFINITY, fs::File, io::Write, path::Path};

use hittable::{HitRecord, Hittable};
use image_convert::{to_png, ImageResource, PNGConfig};
use indicatif::{ProgressBar, ProgressStyle};
use material::scatter;
use ray::Ray;
use renderer::ray_color;
use vec3::functions::{dot, unit_vec};

use crate::{
    hittable::world::World,
    state::State,
    utils::{random_float, write_color},
    vec3::{Color, Point3},
};

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod state;
mod utils;
mod vec3;

#[cfg(test)]
mod tests;

fn main() {
    // World and Camera initialization
    let mut world = World::default();
    let state = State::from_json("state.json");

    world.add_vec(state.entities_vec.clone());

    let camera = state.camera;

    // Render

    let pb = ProgressBar::new(state.height as u64);
    let sty =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix} {bar:40.cyan/blue} [{msg}]")
            .unwrap();

    pb.set_style(sty);

    for frame in 0..state.frames {
        if state.frames > 1 {
            pb.set_prefix(format!("[Frame {}/{}]", frame + 1, state.frames));
        }
        let mut file = File::create(format!("./data/{:04}.ppm", frame)).unwrap();
        file.write_all(format!("P3\n{} {}\n255\n", state.width.unwrap(), state.height).as_bytes())
            .unwrap();

        for j in (0..state.height).rev() {
            pb.inc(1);
            pb.set_message(format!("{:.3}%", (state.height - j) * 100 / state.height));
            for i in 0..state.width.unwrap() {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..state.samples_per_pixel {
                    let u = (i as f64 + random_float()) / (state.width.unwrap() - 1) as f64;
                    let v = (j as f64 + random_float()) / (state.height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(r, &mut world, state.max_depth);
                }
                write_color(pixel_color, state.samples_per_pixel, &mut file);
            }
        }
        pb.reset();
    }
    pb.finish_with_message("Rendering complete!");

    if state.frames == 1 {
        let source_image_path = Path::new("data/0000.ppm");
        let target_image_path = Path::join(source_image_path.parent().unwrap(), "render.png");
        let config = PNGConfig::new();
        let input = ImageResource::from_path(source_image_path);
        let mut output = ImageResource::from_path(target_image_path);
        to_png(&mut output, &input, &config).unwrap()
    }
}
