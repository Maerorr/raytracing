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

    let mut scene = Scene::new();
    let red_sphere = Sphere::new(Vector::new(128.0, 0.0, -200.0), 100.0);
    let blue_sphere = Sphere::new(Vector::new(0.0, 0.0, -100.0), 100.0);
    scene.add_primitive(Box::new(red_sphere), 0);
    scene.add_primitive(Box::new(blue_sphere), 1);

    //let cube_color: Color = Color::new(255, 0, 0, 255);

    //let mut img = ImageBuffer::new(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
    camera.render_scene(&scene, "output.png");

    // for hit in hits.iter() {
    //     if hit.is_some() {
    //         let color_value = {
    //             let angle_cos = hit.angle().cos();
    //             if angle_cos >= 0.0 {
    //                 angle_cos.sqrt()
    //             } else {
    //                 angle_cos.abs().sqrt()
    //             }
    //         };

    //         let color = Color::new(
    //             ((color_value) * cube_color.r as f64) as u8,
    //             ((color_value) * cube_color.g as f64) as u8,
    //             ((color_value) * cube_color.b as f64) as u8,
    //             255);
    //         let (i, mut j) = hit.pos_on_screen;
    //         j = -j;

    //         img.put_pixel((i + OFFSET.0) as u32, (j + OFFSET.1) as u32, image::Rgba([color.r, color.g, color.b, color.a]));

    //     }
    // }

    // if camera.perspective {
    //     // flip img vertically and horizontally
    //     let mut flipped_img = ImageBuffer::new(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
    //     for i in 0..img.width() {
    //         for j in 0..img.height() {
    //             let pixel = img.get_pixel(i as u32, j as u32);
    //             flipped_img.put_pixel((img.width() - i - 1) as u32, (img.height() - j - 1) as u32, *pixel);
    //         }
    //     }
    //     flipped_img.save("outputflip.png").unwrap();
    // } else {
    //     img.save("output.png").unwrap();
    // }   
}