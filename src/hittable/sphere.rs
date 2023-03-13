use serde::{Deserialize, Serialize};

use super::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::functions::dot;
use crate::vec3::Point3;

use super::Hittable;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.dir().norm_squared();
        let half_b = dot(oc, r.dir());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = r.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = dot(r.dir(), normal) < 0.0;

                    let (u, v) = u_v_from_sphere_hit_point(p - self.center);

                    return Some(HitRecord {
                        t: *root,
                        p,
                        normal: if front_face { normal } else { -normal },
                        front_face,
                        material: self.material,
                    });
                }
            }
        }
        None
    }
}

fn u_v_from_sphere_hit_point(hit_point_on_sphere: Point3) -> (f64, f64) {
    let n = hit_point_on_sphere.unit_vec();
    let x = n.x();
    let y = n.y();
    let z = n.z();
    let u = (x.atan2(z) / (2.0 * std::f64::consts::PI)) + 0.5;
    let v = y * 0.5 + 0.5;
    (u, v)
}
