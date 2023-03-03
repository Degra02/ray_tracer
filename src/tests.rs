use std::{fs::File, io::Read};

use crate::hittable::sphere::Sphere;



#[test]
pub fn test_serde() {
   use crate::{vec3::{Point3, Color}, material::Material};

   let vec = vec![Point3::new(0., 0., 0.), Point3::new(1., 1., 0.)];
   let ser_vec = serde_json::to_string(&vec).unwrap();   
   println!("Ser Vec: {}", ser_vec);


   // Point3
   let p = Point3::new(1.0, 1.0, 1.0); 
   let serialized = serde_json::to_string(&p).unwrap();
   println!("Point3: {}", serialized);

   let deserialized: Point3 = serde_json::from_str(&serialized).unwrap();
   println!("Deserialized Point3: {:?}", deserialized);

   // Material
   let material = Material::new_metal(Color::new(0.1, 0.8, 0.3), 0.7);
   let ser_mat = serde_json::to_string(&material).unwrap();
   println!("Ser Mat: {}", ser_mat);

   let de_mat: Material = serde_json::from_str(&ser_mat).unwrap();
   println!("De Mat: {:?}", de_mat);

   // Sphere
   let sphere = Sphere::new(p, 1.0, material); 
   let ser_sphere = serde_json::to_string(&sphere).unwrap();
   println!("Ser Sphere: {}", ser_sphere);

   let de_sphere: Sphere = serde_json::from_str(&ser_sphere).unwrap();
   println!("De Sphere: {:?}", de_sphere);

}


#[test]
pub fn test_serialization() {
   let mut file = File::open("world.json").unwrap();
   let mut to_parse = String::new(); 
   let _res = file.read_to_string(&mut to_parse);
   println!("{}", to_parse);

   let deserialized: Vec<Sphere> = serde_json::from_str(&to_parse).unwrap(); 

   println!("{:?}", deserialized);
}