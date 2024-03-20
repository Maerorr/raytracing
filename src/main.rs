use std::fs::File;
use std::io::Write;
use color::Color;
use float_cmp::{approx_eq, F64Margin};
use geometry::{create_box_surfaces, Sphere, Surface};
use image::{DynamicImage, ImageBuffer};
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

    let material_red = Material::new(Color::new(0.95, 0.1, 0.05));
    let material_blue = Material::new(Color::new(0.0, 0.1, 0.95));

    camera.add_material(material_red);
    camera.add_material(material_blue);

    camera.perspective = true;
    camera.supersampling = false;
    camera.pinhole_distance = 320.0;

    let mut scene = Scene::new();
    let red_sphere = Sphere::new(Vector::new(228.5, 0.0, -300.0), 100.0);
    let blue_sphere = Sphere::new(Vector::new(0.5, 0.0, -100.0), 100.0);
    scene.add_primitive(Box::new(red_sphere), 0);
    scene.add_primitive(Box::new(blue_sphere), 1);

    camera.render_scene(&scene, "output"); 
}