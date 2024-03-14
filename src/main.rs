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
use crate::intersection::Intersection;
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

pub fn zadanie1() {
    let v1 = Vector::new(0.0, 3.0, 0.0);
    let v2 = Vector::new(5.0, 5.0, 0.0);
    let anglev1v2 = v1.angle_degrees(&v2);
    println!("angle: {}", anglev1v2); // checks

    let v1 = Vector::new(4.0, 5.0, 1.0);
    let v2 = Vector::new(4.0, 1.0, 3.0);
    let mut perpendicular = v1.cross(&v2);
    println!("perpendicular: {}", perpendicular.to_string()); // checks
    perpendicular.normalize();
    println!("perpendicular normalized: {}", perpendicular.to_string()); // checks

    let sphere = Sphere::new(
        Vector::new(0.0, 0.0, 0.0),
        10.0,
        Vector::new(255.0, 0.0, 0.0)
    );

    let ray = Line::new(
        Vector::new(0.0, 0.0, -20.0),
        Vector::new(0.0, 0.0, 1.0)
    );

    let ray_y = Line::new(
        Vector::new(0.0, 0.0, -20.0),
        Vector::new(0.0, 1.0, 0.0)
    );

    let hit1 = sphere.intersect(&ray);
    let hit2 = sphere.intersect(&ray_y);
    println!("hit1: {:?}", hit1.hit);
    println!("hit2: {:?}", hit2.hit);

    let ray_single_point_hit = Line::new(
        Vector::new(0.0, 10.0, -20.0),
        Vector::new(0.0, 0.0, 1.0)
    );
    let hit3 = sphere.intersect(&ray_single_point_hit);
    println!("hit3: {:?}", hit3.hit);

    let v = Vector::new(1.0, 0.0, 0.0); // x axis
    let mut w = Vector::new(0.0, -1.0, 1.0); // half y half z -> 45 degrees
    w.normalize();
    let mut norm = v.cross(&w);
    norm.normalize();
    let surface = Surface::new_vw(Vector::new(0.0, 0.0, 0.0), v, w, None, None, norm);
    let hit_surf = surface.intersect(&ray_y);
    println!("hit_surf: {:?}", hit_surf.hit);

    let triangle = Triangle::new(
        [Vector::new(0.0, 0.0, 0.0),
        Vector::new(1.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0)],
        Vector::new(0.0, 255.0, 0.0)
    );

    let start = Vector::new(-1.0, 0.5, -0.5);
    let end = Vector::new(1.0, 0.5, -0.5);
    let mut dir = end - start;
    dir.normalize();

    let tri_ray = Line::new(
        start,
        dir
    );

    let hit_tri = triangle.intersect(&tri_ray);
    println!("hit_tri: {:?}", hit_tri.hit);
}

fn main() {
    zadanie1();
    return;
    let mut scene = Scene::new();

    let mut q = Quaternion::identity();

    q.rotate(as_radians(15.0), Vector::new(1.0, 0.0, 0.0));
    q.rotate(as_radians(15.0), Vector::new(0.0, 1.0, 0.0));
    q.rotate(as_radians(15.0), Vector::new(0.0, 0.0, 1.0));

    let mut front = Surface::new_vw(
        Vector::new(0.0, 0.0, 15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(0.0, 0.0, -1.0));
    let mut back = Surface::new_vw(
        Vector::new(0.0, 0.0, -15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(0.0, 0.0, 1.0));
    let mut left = Surface::new_vw(
        Vector::new(-15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(1.0, 0.0, 0.0));
    let mut right = Surface::new_vw(
        Vector::new(15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(-1.0, 0.0, 0.0));
    let mut top = Surface::new_vw(
        Vector::new(0.0, 15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(0.0, -1.0, 0.0));
    let mut bottom = Surface::new_vw(
        Vector::new(0.0, -15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Some((-15.0, 15.0)),
        Some((-15.0, 15.0)),
        Vector::new(0.0, 1.0, 0.0));

    
    scene.add_surface(front);
    scene.add_surface(back);
    scene.add_surface(left);
    scene.add_surface(right);
    scene.add_surface(top);
    scene.add_surface(bottom);

    for surface in scene.surfaces.iter_mut() {
        surface.rotate(&q);
        surface.translate(&Vector::new(-150.0, 00.0, 0.0));
    }

    let sphere = Sphere::new(
        Vector::new(190.0, 0.0, 0.0),
        20.0,
        Vector::new(255.0, 0.0, 0.0)
    );

    scene.add_sphere(sphere);

    let triangle = Triangle::new(
        [Vector::new(-100.0, 0.0, 0.0),
        Vector::new(-70.0, 40.0, 0.0),
        Vector::new(-30.0, 0.0, 0.0)],
        Vector::new(0.0, 255.0, 0.0)
    );

    //println!("{:?}", triangle.normal);

    scene.add_triangle(triangle);

    let triangle = Triangle::new(
        [Vector::new(100.0, 0.0, 0.0),
        Vector::new(70.0, 40.0, 0.0),
        Vector::new(30.0, 0.0, 100.0)],
        Vector::new(0.0, 255.0, 0.0)
    );

    //println!("{:?}", triangle.normal);

    scene.add_triangle(triangle);

    let mut hits: Vec<RayCastHit> = Vec::new();

    let mut q: Quaternion = Quaternion::identity();

    let camera_pos = Vector::new(0.0, 0.0, -100.0);

    let mut camera = Camera::new(
        camera_pos.clone(),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 0.0));

    camera.perspective = true;

    let cube_color: Color = Color::new(255, 0, 0, 255);

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
            j = -j;

            img.put_pixel((i + OFFSET.0) as u32, (j + OFFSET.1) as u32, image::Rgba([color.r, color.g, color.b, color.a]));

        }
    }

    if camera.perspective {
        // flip img vertically and horizontally
        let mut flipped_img = ImageBuffer::new(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
        for i in 0..img.width() {
            for j in 0..img.height() {
                let pixel = img.get_pixel(i as u32, j as u32);
                flipped_img.put_pixel((img.width() - i - 1) as u32, (img.height() - j - 1) as u32, *pixel);
            }
        }
        flipped_img.save("outputflip.png").unwrap();
    } else {
        img.save("output.png").unwrap();
    }   
}