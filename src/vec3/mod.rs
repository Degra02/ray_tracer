pub mod functions;
pub mod utils;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3]
}

pub type Color = Vec3;
pub type Point3 = Vec3;
