use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::{
    camera::{Camera},
    hittable::sphere::Sphere,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub aspect_ratio: f64,
    pub width: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    pub frames: u32,

    pub camera: Camera,
    pub entities_vec: Vec<Sphere>,
}

impl State {
    pub fn new(
        aspect_ratio: f64,
        width: u32,
        height: Option<i32>,
        frames: u32,
        camera: Camera,
        entities_vec: Vec<Sphere>,
    ) -> Self {
        Self {
            aspect_ratio,
            width,
            height,
            frames,
            camera,
            entities_vec,
        }
    }

    pub fn from_json(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut to_parse = String::new();
        let _res = file.read_to_string(&mut to_parse);
        // println!("{}", to_parse);

        let mut state: State = serde_json::from_str(&to_parse).unwrap();

        if state.height.is_none() {
            state.height = Some((state.width as f64 / state.aspect_ratio) as i32);
        }

        state
    }
}
