use super::Vec3; 

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() + v2.z()
}
