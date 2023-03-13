use std::{fs::File, io::Read};

use palette::Srgb;

use crate::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::{Lambertian, Metal},
    state::State,
    vec3::{Color, Point3, Vec3},
};

#[test]
pub fn test_serde() {
    use crate::{material::Material, vec3::Point3};

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
    let material = Material::new_metal(Srgb::new(0.1, 0.8, 0.3), 0.7);
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
pub fn test_vec_serde() {
    let mut file = File::open("world.json").unwrap();
    let mut to_parse = String::new();
    let _res = file.read_to_string(&mut to_parse);
    println!("{}", to_parse);

    let deserialized: Vec<Sphere> = serde_json::from_str(&to_parse).unwrap();

    println!("{:?}", deserialized);
}

#[test]
pub fn camera_serde() {
    let aspect_ratio = 1.0 / 9.0;

    let look_from = Point3::new(0., 1., 1.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(look_from, look_at, vup, 50.0, aspect_ratio);

    let camera_str = serde_json::to_string(&camera).unwrap();
    println!("Camera Ser: {}", camera_str);

    let camera_deser: Camera = serde_json::from_str(&camera_str).unwrap();
    println!("Camera Deser: {:?}", camera_deser);
}

#[test]
pub fn state_serde() {
    let aspect_ratio = 16.0 / 9.0;
    let height = 900u32;
    let width: Option<i32> = None;
    let frames = 1u32;

    let look_from = Point3::new(0., 1., 1.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(look_from, look_at, vup, 50.0, aspect_ratio);

    let sphere1 = Sphere::new(
        Point3::new(0., 0., 0.),
        0.5,
        crate::material::Material::Metal(Metal {
            albedo: Srgb::new(1.0, 0.2, 0.2),
            fuzz: 1.0,
        }),
    );

    let sphere2 = Sphere::new(
        Point3::new(0., 0., 0.),
        0.5,
        crate::material::Material::Lambertian(Lambertian {
            albedo: Srgb::new(1.0, 0., 0.7),
        }),
    );

    let entities_vec = vec![sphere1, sphere2];

    let state = State::new(
        100,
        50,
        aspect_ratio,
        width,
        height,
        frames,
        camera,
        entities_vec,
    );
    let state_ser = serde_json::to_string(&state).unwrap();
    println!("State Ser: {}", state_ser);

    let state_deser: State = serde_json::from_str(&state_ser).unwrap();
    println!("State Deser: {:?}", state_deser);
}

#[test]
fn init_from_file() {
    let state_ser = State::from_json("state.json");
    println!("State Ser: {:?}", state_ser);
}
