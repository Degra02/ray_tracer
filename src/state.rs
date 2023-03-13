use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::{camera::Camera, hittable::sphere::Sphere};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub aspect_ratio: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    pub height: u32,
    pub frames: u32,

    pub camera: Camera,
    pub entities_vec: Vec<Sphere>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<Sphere>>,
}

impl State {
    pub fn new(
        samples_per_pixel: i32,
        max_depth: i32,
        aspect_ratio: f64,
        width: Option<i32>,
        height: u32,
        frames: u32,
        camera: Camera,
        entities_vec: Vec<Sphere>,
        lights: Vec<Sphere>
    ) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
            aspect_ratio,
            width,
            height,
            frames,
            camera,
            entities_vec,
            lights: Some(lights)
        }
    }

    pub fn from_json(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut to_parse = String::new();
        let _res = file.read_to_string(&mut to_parse);
        // println!("{}", to_parse);

        let mut state: State = serde_json::from_str(&to_parse).unwrap();

        if state.width.is_none() {
            state.width = Some((state.height as f64 * state.aspect_ratio) as i32);
        }

        state
    }
}
