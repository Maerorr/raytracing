use std::fs::File;
use std::io::Write;
use camera::AntiAliasingType;
use color::Color;
use float_cmp::{approx_eq, F64Margin};
use geometry::{create_box_surfaces, Sphere, Surface};
use image::{DynamicImage, ImageBuffer};
use light::Light;
use material::Material;
use math::{Quaternion, RayCastHit, Vector};
use scene::Scene;

use crate::math::{as_degrees, as_radians};
use crate::camera::Camera;

use crate::geometry::{Line, Triangle};

mod camera;
mod scene;
mod material;
mod color;
mod buffer;
mod light;

mod geometry;
mod math;

const RENDER_WIDTH: i32 = 512;
const RENDER_HEIGHT: i32 = 512;

const OFFSET: (i32, i32) = (RENDER_WIDTH / 2, RENDER_HEIGHT / 2);

pub fn save_to_file(hits: &Vec<RayCastHit>) {
    let mut file = File::create("output.txt").unwrap();

    for (i, hit) in hits.iter().enumerate() {
        if hit.is_some() {
            file.write(b"0").unwrap();
        } else {
            file.write(b".").unwrap();
        }
        if (i + 1) % RENDER_WIDTH as usize == 0 {
            file.write(b"\n").unwrap();
        }

    }
}

pub fn display_debug(c: &Camera) {
    println!("{}", c.get_debug_info());
}

fn main() {
    let mut camera = Camera::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0)
    );

    let material_red_specular = Material::new(
        Color::new(0.9, 0.23, 0.11),
        0.8,
        16.0,
    );
    let material_blue_matte = Material::new(
        Color::new(0.0, 0.1, 0.95),
        0.1,
        4.0,
    );

    camera.add_material(material_red_specular);
    camera.add_material(material_blue_matte);

    camera.perspective = true;
    camera.aa_type = AntiAliasingType::AdaptiveX;
    camera.pinhole_distance = 320.0;

    let mut scene = Scene::new();
    // let red_sphere = Sphere::new(Vector::new(128.0, 0.0, -300.0), 100.0);
    // let blue_sphere = Sphere::new(Vector::new(0.0, 0.0, -100.0), 100.0);
    // scene.add_primitive(Box::new(red_sphere), 0);
    // scene.add_primitive(Box::new(blue_sphere), 1);

    // create 8 by 8 grid of spheres at y = -128, with radius of 25 and spacing of 75
    let y = -225.0;
    let radius = 25.0;
    let spacing = 75.0;
    let mut material = 0;
    for i in 0..8 {
        for j in 0..9 {
            let x = (i as f64 - 3.5) * spacing;
            let z = (j as f64 - 3.5) * spacing - 300.0;
            let sphere = Sphere::new(Vector::new(x, y, z), radius);
            scene.add_primitive(Box::new(sphere), material);
            material ^= 1;
        }
    }

    let ambient = Light::new_ambient(Color::new(0.1, 0.1, 0.1));
    scene.add_light(ambient);

    camera.render_scene(&scene, "output"); 
    camera.antialias_debug_buffer.save("aa_debug.png");
}