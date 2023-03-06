use serde::{Deserialize, Serialize, de::{Visitor, self, MapAccess}};

use crate::{
    ray::Ray,
    utils::deg_to_rad,
    vec3::{
        functions::{cross, unit_vec},
        Point3, Vec3,
    },
};

#[derive(Debug, Serialize)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = f64::tan(theta / 2.);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = unit_vec(look_from - look_at);
        let u = unit_vec(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        enum Field { LookFrom, LookAt, Vup, Vfov, AspectRatio }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
            D: serde::Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("look_from, look_at, vup, vfov, aspect_ratio") 
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E> 
                    where E: de::Error {
                       match value {
                           "look_from" => Ok(Field::LookFrom),
                           "look_at" => Ok(Field::LookAt),
                           "vup" => Ok(Field::Vup),
                           "vfov" => Ok(Field::Vfov),
                           "aspect_ratio" => Ok(Field::AspectRatio),
                           _ => Err(de::Error::unknown_field(value, FIELDS)) 
                       } 
                    }
                }
           
                deserializer.deserialize_identifier(FieldVisitor)  
            }

        }

        struct CameraVisitor;

        impl<'de> Visitor<'de> for CameraVisitor {
            type Value = Camera;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Camera")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>, {
                let look_from = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let look_at = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let vup = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let vfov = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let aspect_ratio = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(4, &self))?;

                Ok(Camera::new(look_from, look_at, vup, vfov, aspect_ratio))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Camera, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut look_at = None;
                let mut look_from = None;
                let mut vup = None;
                let mut vfov = None;
                let mut aspect_ratio = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LookAt => {
                            if look_at.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            look_at = Some(map.next_value()?);
                        }
                        Field::LookFrom => {
                            if look_from.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            look_from = Some(map.next_value()?);
                        }
                        
                        Field::Vup => {
                            if vup.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            vup = Some(map.next_value()?);
                        }
                        
                        Field::Vfov => {
                            if vfov.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            vfov = Some(map.next_value()?);
                        }
                        
                        Field::AspectRatio => {
                            if aspect_ratio.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            aspect_ratio = Some(map.next_value()?);
                        }
                    }
                }
                let look_from = look_from.ok_or_else(|| de::Error::missing_field("look_from"))?;
                let look_at = look_at.ok_or_else(|| de::Error::missing_field("look_at"))?;
                let vup = vup.ok_or_else(|| de::Error::missing_field("vup"))?;
                let vfov = vfov.ok_or_else(|| de::Error::missing_field("vfov"))?;
                let aspect_ratio = aspect_ratio.ok_or_else(|| de::Error::missing_field("aspect_ratio"))?;
                Ok(Camera::new(look_from, look_at, vup, vfov, aspect_ratio))
            }
        }

        const FIELDS: &[&str] = &["look_from", "look_at", "vup", "vfov", "aspect_ratio"];
        deserializer.deserialize_struct("Camera", FIELDS,CameraVisitor)
    }
}
