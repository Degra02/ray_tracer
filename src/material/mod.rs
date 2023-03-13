use palette::Srgb;
use serde::{Deserialize, Serialize};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::random_float,
    vec3::{
        functions::{dot, reflect, refract, unit_vec},
        Vec3,
    },
};

serde_with::serde_conv!(
    SrgbAsArray,
    Srgb,
    |srgb: &Srgb| [srgb.red, srgb.green, srgb.blue],
    |value: [f32; 3]| -> Result<_, std::convert::Infallible> {
        Ok(Srgb::new(value[0], value[1], value[2]))
    }
);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Light(Light),
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)>;
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian {
            albedo: Srgb::default(),
        })
    }
}

impl Material {
    pub fn new_lambertian(albedo: Srgb) -> Self {
        Self::Lambertian(Lambertian { albedo })
    }

    pub fn new_metal(albedo: Srgb, fuzz: f64) -> Self {
        Self::Metal(Metal { albedo, fuzz })
    }

    pub fn new_dielectric(ir: f64) -> Self {
        Self::Dielectric(Dielectric { ir })
    }
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
            Material::Light(l) => l.scatter(ray, hit_record),
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Light {}

impl Light {
    pub fn new() -> Self {
        Self {  }
    }
}

#[allow(unused)]
impl Scatterable for Light {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
       Some((None, Srgb::new(1.0, 1.0, 1.0))) 
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Lambertian {
    #[serde_as(as = "SrgbAsArray")]
    pub albedo: Srgb,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((Some(scattered), attenuation))
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metal {
    #[serde_as(as = "SrgbAsArray")]
    pub albedo: Srgb,
    pub fuzz: f64,
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let reflected = reflect(unit_vec(ray.dir()), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if dot(scattered.dir(), hit_record.normal) > 0. {
            Some((Some(scattered), attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Dielectric {
    pub ir: f64,
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let attenuation = Srgb::new(1., 1., 1.);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vec(ray.dir());

        let cos_theta = f64::min(dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
            let reflected = reflect(unit_direction, hit_record.normal);
            let scattered = Ray::new(hit_record.p, reflected);
            Some((Some(scattered), attenuation))
        } else {
            let direction = refract(unit_direction, hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.p, direction);
            Some((Some(scattered), attenuation))
        }
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * f64::powi(1. - cosine, 5)
}

// pub fn scatter(
//     material: Material,
//     ray_in: Ray,
//     rec: HitRecord,
//     attenuation: &mut Color,
//     scattered: &mut Ray,
// ) -> bool {
//     match material {
//         Material::Lambertian { albedo } => {
//             let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
//             if scatter_direction.near_zero() {
//                 scatter_direction = rec.normal
//             }
//             *scattered = Ray::new(rec.p, scatter_direction);
//             *attenuation = albedo;
//             true
//         }
//         Material::Metal { albedo, fuzz } => {
//             let reflected = reflect(unit_vec(ray_in.dir()), rec.normal);
//             *scattered = Ray::new(
//                 rec.p,
//                 reflected + fuzz * Vec3::random_in_hemisphere(rec.normal),
//             );
//             *attenuation = albedo;
//             dot(scattered.dir(), rec.normal) > 0.
//         }
//         Material::Dielectric { ir } => {
//             *attenuation = Color::new(1., 1., 1.);
//             let refraction_ratio = if rec.front_face { 1.0 / ir } else { ir };

//             let unit_direction = unit_vec(ray_in.dir());

//             let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
//             let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

//             let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
//             let direction: Vec3;

//             if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
//                 direction = reflect(unit_direction, rec.normal);
//             } else {
//                 direction = refract(unit_direction, rec.normal, refraction_ratio);
//             }

//             *scattered = Ray::new(rec.p, direction);
//             true
//         }
//         Material::Light {} => {
//             *attenuation = Color::new(1., 1., 1.);
//             true
//         }
//     }
// }
