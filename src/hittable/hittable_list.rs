use super::{sphere::Sphere, HitRecord, Hittable};
use crate::ray::Ray;


#[derive(Default)]
pub struct HittableList {
    objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new(objects: Vec<Sphere>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn add_vec(&mut self, vector: Vec<Sphere>) {
        for obj in vector {
            self.objects.push(obj)
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.clone().t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
