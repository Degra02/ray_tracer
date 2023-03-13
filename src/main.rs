#![allow(dead_code)]
use std::{f64::INFINITY, fs::File, io::Write, path::Path};

use hittable::{HitRecord, Hittable};
use image_convert::{to_png, ImageResource, PNGConfig};
use indicatif::{ProgressBar, ProgressStyle};
use material::scatter;
use ray::Ray;
use renderer::{ray_color, render};
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

    // Render

    let pb = ProgressBar::new(state.height as u64);
    let sty =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix} {bar:40.cyan/blue} [{msg}]")
            .unwrap();

    render(state);
}
