#![allow(dead_code)]

use indicatif::{ProgressBar, ProgressStyle};
use renderer::render;
use crate::{
    state::State
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
    let state = State::from_json("state.json");

    // Render

    let pb = ProgressBar::new(state.height as u64);
    let sty =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix} {bar:40.cyan/blue} [{msg}]")
            .unwrap();

    render(state);
}
