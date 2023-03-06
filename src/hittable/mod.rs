pub mod hittable_list;
pub mod sphere;

// use std::{cell::RefCell, rc::Rc};

use crate::{
    material::Material,
    ray::Ray,
    vec3::{functions::dot, Point3, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.normal = match dot(r.dir(), outward_normal) < 0. {
            true => outward_normal,
            false => -outward_normal,
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
