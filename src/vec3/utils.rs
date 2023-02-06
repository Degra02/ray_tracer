use super::{Color, Vec3};

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() + v2.z()
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        (255.999 * color.x()) as i32,
        (255.999 * color.y()) as i32,
        (255.999 * color.z()) as i32
    )
}

pub fn unit_vec(vec: Vec3) -> Vec3 {
    vec / vec.norm()
}
