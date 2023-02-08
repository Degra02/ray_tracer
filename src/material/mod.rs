use crate::{vec3::{Color, Vec3, functions::{reflect, unit_vec, dot}}, hittable::HitRecord, ray::Ray};


#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian {
        albedo: Color
    },
    Metal {
        albedo: Color,
        fuzz: f32,
    },
    Dielectric{}
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: Color::default()
        }
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
            *scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
            *attenuation = albedo;
            return dot(scattered.dir(), rec.normal) > 0.;
        },
        Material::Dielectric {  } => {},
    }


    true
}

