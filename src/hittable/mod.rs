pub mod sphere;
pub mod world;

// use std::{cell::RefCell, rc::Rc};

use crate::{
    material::Material,
    ray::Ray,
    vec3::{functions::dot, Point3, Vec3},
};

use self::sphere::Sphere;

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

pub fn hit_world(world: &Vec<Sphere>, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut rec = HitRecord::default();

    for obj in world.iter() {
        if obj.hit(r.clone(), t_min, closest_so_far, &mut rec) {
            closest_so_far = rec.clone().t;
        }
    }

    Some(rec)
}
