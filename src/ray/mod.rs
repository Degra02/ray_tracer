pub mod functions;

use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}
