#![allow(dead_code)]

use crate::state::State;
use indicatif::{ProgressBar, ProgressStyle};
use renderer::render;

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

    let _pb = ProgressBar::new(state.height as u64);
    let _sty =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix} {bar:40.cyan/blue} [{msg}]")
            .unwrap();

    render(state);
}
