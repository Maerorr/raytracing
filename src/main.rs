use std::fs::File;
use std::io::Write;
use float_cmp::{approx_eq, F64Margin};
use image::{DynamicImage, ImageBuffer};
use raylib::ffi::ImageFormat;
use scene::Scene;
use sphere::Sphere;
use triangle::Triangle;
use vector::*;
use point::*;
use mat4::*;
use crate::line::Line;
use crate::math::{as_degrees, as_radians};
use crate::quaternion::Quaternion;
use crate::surface::Surface;
use crate::camera::Camera;

use raylib::prelude::*;
use cstr::cstr;
use crate::object::Object;
use crate::raycasthit::RayCastHit;

mod vector;
mod point;
mod mat4;
mod math;
mod quaternion;
mod surface;
mod line;
mod object;
mod camera;
mod raycasthit;
mod intersection;
mod sphere;
mod scene;
mod triangle;

const RENDER_WIDTH: i32 = 512;
const RENDER_HEIGHT: i32 = 256;

const OFFSET: (i32, i32) = (RENDER_WIDTH / 2, RENDER_HEIGHT / 2);

// background color
static BG_COLOR: Color = Color {
    r: 0,
    g: 172,
    b: 210,
    a: 255,
};

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

pub fn draw_slider(d: &mut RaylibDrawHandle, text: String, x: i32, y: &mut i32, value: &f32, range: (f32, f32)) -> f32 {
    d.draw_text(text.as_str(), x, *y, 32, Color::WHITE);

    let out = d.gui_slider_bar(
        Rectangle::new((x + 175) as f32, *y as f32, 300.0, 30.0),
        None,
        None,
        *value,
        range.0, range.1);

    d.draw_text(&format!("{:.2}", out), x + 250, *y, 32, Color::DARKGRAY);
    *y += 50;
    out
}

fn main() {

    let mut vec = Vector::new(1.0, 2.0, 3.0);
    let mut mat = Mat4::identity();
    mat.scale(Vector::new(2.0, 2.0, 2.0));
    println!("{}", mat.to_string());
    let vec2 = vec * mat;
    assert_eq!(vec2, Vector::new(2.0, 4.0, 6.0));
    vec = Vector::new(1.0, 0.0, 0.0);
    let mut mat = Mat4::identity();
    mat.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));
    let vec2 = vec * mat;
    assert_eq!(vec2, Vector::new(0.0, 0.0, -1.0));

    let mut scene = Scene::new();

    let mut q = Quaternion::identity();

    q.rotate(as_radians(15.0), Vector::new(1.0, 0.0, 0.0));
    q.rotate(as_radians(15.0), Vector::new(0.0, 1.0, 0.0));
    q.rotate(as_radians(15.0), Vector::new(0.0, 0.0, 1.0));

    let mut front = Surface::new_vw(
        Vector::new(0.0, 0.0, 15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 0.0, -1.0));
    let mut back = Surface::new_vw(
        Vector::new(0.0, 0.0, -15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 0.0, 1.0));
    let mut left = Surface::new_vw(
        Vector::new(-15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(1.0, 0.0, 0.0));
    let mut right = Surface::new_vw(
        Vector::new(15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(-1.0, 0.0, 0.0));
    let mut top = Surface::new_vw(
        Vector::new(0.0, 15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, -1.0, 0.0));
    let mut bottom = Surface::new_vw(
        Vector::new(0.0, -15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 1.0, 0.0));

    
    scene.add_surface(front);
    scene.add_surface(back);
    scene.add_surface(left);
    scene.add_surface(right);
    scene.add_surface(top);
    scene.add_surface(bottom);

    for surface in scene.surfaces.iter_mut() {
        surface.rotate(&q);
    }

    let sphere = Sphere::new(
        Vector::new(60.0, -60.0, 0.0),
        40.0,
        Vector::new(255.0, 0.0, 0.0)
    );

    scene.add_sphere(sphere);

    let triangle = Triangle::new(
        [Vector::new(-100.0, 0.0, 0.0),
        Vector::new(-70.0, 40.0, 0.0),
        Vector::new(-30.0, 0.0, 0.0)],
        Vector::new(0.0, 255.0, 0.0)
    );

    println!("{:?}", triangle.normal);

    scene.add_triangle(triangle);

    let triangle = Triangle::new(
        [Vector::new(100.0, 0.0, 0.0),
        Vector::new(70.0, 40.0, 0.0),
        Vector::new(30.0, 0.0, -100.0)],
        Vector::new(0.0, 255.0, 0.0)
    );

    println!("{:?}", triangle.normal);

    scene.add_triangle(triangle);

    let mut hits: Vec<RayCastHit> = Vec::new();

    let mut q: Quaternion = Quaternion::identity();

    let mut camera_pos = Vector::new(0.0, 0.0, 50.0);

    let mut camera = Camera::new(
        camera_pos.clone(),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 0.0));

    camera.set_perspective(120.0, 1.0, 1000.0);

    let mut cube_color: Color = Color::new(255, 0, 0, 255);

    let mut img = ImageBuffer::new(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
        hits = camera.render_scene(&scene);

        for hit in hits.iter() {
            if hit.is_some() {
                let color_value = {
                    let angle_cos = hit.angle().cos();
                    if angle_cos >= 0.0 {
                        angle_cos.sqrt()
                    } else {
                        angle_cos.abs().sqrt()
                    }
                };

                let color = Color::new(
                    ((color_value) * cube_color.r as f64) as u8,
                    ((color_value) * cube_color.g as f64) as u8,
                    ((color_value) * cube_color.b as f64) as u8,
                    255);
                let (i, mut j) = hit.pos_on_screen;
                //print!("i: {}, j: {}, ", i, j);
                j = -j;

                img.put_pixel((i + OFFSET.0) as u32, (j + OFFSET.1) as u32, image::Rgba([color.r, color.g, color.b, color.a]));

            }
        }

    img.save("output.png").unwrap();
}