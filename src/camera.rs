use serde::{Serialize, Deserialize};

use crate::{vec3::{Point3, Vec3, functions::{unit_vec, cross}}, ray::Ray, utils::deg_to_rad};

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = deg_to_rad(vfov);
        let h = f64::tan(theta/2.);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = unit_vec(look_from - look_at);
        let u = unit_vec(cross(vup, w));
        let v = cross(w, u);


        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, 
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }

}
