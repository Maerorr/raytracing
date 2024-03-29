use std::fs::File;
use std::io::Write;
use camera::AntiAliasingType;
use color::Color;
use float_cmp::{approx_eq, F64Margin};
use geometry::{create_box_surfaces, Sphere, Surface};
use image::{DynamicImage, ImageBuffer};
use light::{Light, RectangleAreaLight};
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

const RED_MAT: usize = 0;
const BLUE_MAT: usize = 1;
const WHITE_MAT: usize = 2;
const GREEN_MAT: usize = 3;

fn main() {
    let mut camera = Camera::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0)
    );

    let material_red_specular = Material::new(
        Color::new(0.9, 0.23, 0.11),
        0.9,
        24.0,
    );
    let material_blue_matte = Material::new(
        Color::new(0.0, 0.1, 0.95),
        0.1,
        4.0,
    );
    let white_material = Material::new(
        Color::white(),
        0.01,
        4.0,
    );

    let green_mat = Material::new(
        Color::green() * 0.8,
        0.8,
        16.0,
    );

    camera.add_material(material_red_specular);
    camera.add_material(material_blue_matte);
    camera.add_material(white_material);
    camera.add_material(green_mat);

    let mut scene = Scene::new();

    let floor = Surface::new_vw(
        Vector::new(0.0, -300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, 1.0, 0.0)
    );
    scene.add_primitive(Box::new(floor), BLUE_MAT);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -500.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(back_wall), RED_MAT);

    let left_wall = Surface::new_vw(
        Vector::new(-300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(1.0, 0.0, 0.0)
    );
    scene.add_primitive(Box::new(left_wall), RED_MAT);

    // let right_wall = Surface::new_vw(
    //     Vector::new(300.0, 0.0, 0.0),
    //     Vector::new(0.0, 0.0, -1.0),
    //     Vector::new(0.0, 1.0, 0.0),
    //     None,
    //     None,
    //     Vector::new(-1.0, 0.0, 0.0)
    // );
    // scene.add_primitive(Box::new(right_wall), RED_MAT);

    let ceiling = Surface::new_vw(
        Vector::new(0.0, 300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, -1.0, 0.0)
    );
    scene.add_primitive(Box::new(ceiling), BLUE_MAT);

    let sphere = Sphere::new(Vector::new(-125.0, -220.0, -100.0), 50.0);
    scene.add_primitive(Box::new(sphere), BLUE_MAT);

    let sphere2 = Sphere::new(Vector::new(-125.0, 100.0, -200.0), 50.0);
    scene.add_primitive(Box::new(sphere2), RED_MAT);

    let sphere3 = Sphere::new(Vector::new(0.0, -200.0, -240.0), 50.0);
    scene.add_primitive(Box::new(sphere3), GREEN_MAT);

    let ambient = Light::new_ambient(Color::white(), 0.11);
    scene.add_light(ambient);

    // let point = Light::new_point(
    //     Vector::new(0.0, 0.0, -50.0),
    //     Color::white(), 
    //     (0.45, 0.0009, 0.00001));
    // scene.add_light(point);

    let area_light = RectangleAreaLight::new(
        Vector::new(380.0, 0.0, -150.0),
        Color::white(),
        ((1.0 / 64.0), 0.002, 0.0001),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        100.0,
        100.0,
        6.0,
    );
    scene.add_lights(area_light.get_lights());

    camera.perspective = true;
    camera.aa_type = AntiAliasingType::Supersampling4x;
    camera.pinhole_distance = 320.0;

    //camera.render_scene(&scene, "output"); 
    camera.render_scene_multithreaded(scene, "multithread.png");
    camera.antialias_debug_buffer.save("aa_debug.png");
}