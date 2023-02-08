use std::cmp::min;

use crate::{vec3::{Color, Vec3, functions::{reflect, unit_vec, dot, refract}}, hittable::HitRecord, ray::Ray, utils::random_float};


#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian {
        albedo: Color
    },
    Metal {
        albedo: Color,
        fuzz: f32,
    },
    Dielectric{
        ir: f32
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: Color::default()
        }
    }
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian {
            albedo
        }
    }

    pub fn new_metal(albedo: Color, fuzz: f32) -> Self {
        Self::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(ir: f32) -> Self {
        Self::Dielectric { ir }
    }
}

pub fn scatter(material: Material, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    match material {
        Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal
            }
            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = albedo;
            return true;
        },
        Material::Metal { albedo, fuzz } => {
           let reflected = reflect(unit_vec(ray_in.dir()), rec.normal); 
            *scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_hemisphere(rec.normal));
            *attenuation = albedo;
            return dot(scattered.dir(), rec.normal) > 0.;
        },
        Material::Dielectric { ir } => {
            *attenuation = Color::new(1., 1., 1.);
            let refraction_ratio = if rec.front_face {
                1.0 / ir  
            } else {
                ir
            };

            let unit_direction = unit_vec(ray_in.dir());

            let cos_theta = f32::min(dot(-unit_direction, rec.normal), 1.0);
            let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);
            
            let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
            let direction: Vec3;

            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float(){
                direction = reflect(unit_direction, rec.normal); 
            } else {
                direction = refract(unit_direction, rec.normal, refraction_ratio);
            }

            *scattered = Ray::new(rec.p, direction); 
            true
        },
    }
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx); 
    r0 *= r0;
    r0 + (1. - r0)* f32::powi(1. - cosine, 5)
}








