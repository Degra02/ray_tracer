use super::{Color, Vec3};

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    )
}

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        (255.999 * color[0]) as u8,
        (255.999 * color[1]) as u8,
        (255.999 * color[2]) as u8
    )
}

pub fn unit_vec(vec: Vec3) -> Vec3 {
    vec / vec.norm()
}
