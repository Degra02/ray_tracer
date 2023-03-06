use serde::{Deserialize, Serialize};

pub mod functions;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;
