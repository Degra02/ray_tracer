pub mod functions;
pub mod utils;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Color = Vec3;
pub type Point3 = Vec3;
