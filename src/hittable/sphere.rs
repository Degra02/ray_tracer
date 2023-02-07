use super::HitRecord;
use crate::ray::Ray;
use crate::vec3::utils::dot;
use crate::vec3::Point3;

use super::Hittable;

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.dir().norm_squared();
        let half_b = dot(oc, r.dir());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        let mut root = (-half_b - f32::sqrt(discriminant)) / a;

        if root < t_min || root > t_max {
            root = (-half_b + f32::sqrt(discriminant)) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
