use std::fs::File;
use std::io::Write;
use camera::AntiAliasingType;
use color::Color;
use float_cmp::{approx_eq};
use geometry::{create_box_surfaces, Sphere, Surface};
use image::{DynamicImage, ImageBuffer};
use light::{Light, RectangleAreaLight};
use material::Material;
use math::{Quaternion, RayCastHit, Vector};
use presentation_scenes::{pbr_scene, reflection_refraction_scene, shading_scene};
use scene::Scene;

use crate::math::{as_degrees, as_radians, IntersectionPrimitive};
use crate::camera::Camera;

use crate::geometry::{Line, Triangle};

mod camera;
mod scene;
mod material;
mod color;
mod buffer;
mod light;
mod presentation_scenes;

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
const BLACK_MAT: usize = 4;
const MIRROR_MAT: usize = 5;
const GLASS_MAT: usize = 6;

fn main() {

    for _ in 0..10 {
        let v = Vector::random(-0.5, 0.5);
        println!("{}", v.to_string());
    }

    let mut camera = Camera::new(
        Vector::new(0.0, -50.0, 100.0),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0)
    );

    // let materials = init_materials();
    // for mat in materials {
    //     camera.add_material(mat);
    // }

    //let scene = reflection_refraction_scene();
    let (scene, materials) = pbr_scene();

    for mat in materials {
        camera.add_material(mat);
    }

    camera.perspective = true;
    camera.aa_type = AntiAliasingType::Supersampling4x;
    camera.pinhole_distance = 390.0;

    //camera.render_scene(&scene, "output"); 
    camera.render_scene_multithreaded(scene, "multithread.png");
    camera.antialias_debug_buffer.save("aa_debug.png");
}

pub fn init_materials() -> Vec<Material> {
    let mut mats = Vec::new();

    let material_red_specular = Material::new_phong(
        Color::new(0.9, 0.23, 0.11),
        0.9,
        128.0,
    );
    mats.push(material_red_specular);
    let material_blue_matte = Material::new_phong(
        Color::new(0.0, 0.1, 0.95),
        0.1,
        4.0,
    );
    mats.push(material_blue_matte);
    let white_material = Material::new_phong(
        Color::white(),
        0.01,
        4.0,
    );
    mats.push(white_material);
    let green_mat = Material::new_phong(
        Color::green() * 0.8,
        0.8,
        8.0,
    );
    mats.push(green_mat);
    let black_mat = Material::new_phong(
        Color::black(),
        0.4,
        2.0,
    );
    mats.push(black_mat);
    let mirror_mat = Material::new_reflective(
        Color::green(),
        0.2,
        32.0,
        10000.0,
    );
    mats.push(mirror_mat);
    let glass_mat = Material::new_refractive(
        Color::white(),
        1.66,
    );
    mats.push(glass_mat);

    mats
}