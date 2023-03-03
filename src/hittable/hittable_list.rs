use super::{HitRecord, Hittable, sphere::Sphere};
use crate::ray::Ray;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<RefCell<Sphere>>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<RefCell<Sphere>>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Rc<RefCell<Sphere>>) {
        self.objects.push(object.clone());
    }

    pub fn add_vec(&mut self, vector: Vec<Rc<RefCell<Sphere>>>) {
       for obj in vector {
            self.objects.push(obj.clone())
       } 
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if (*obj).borrow().hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.clone().t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
