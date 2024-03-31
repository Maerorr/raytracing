use crate::{math::intersection::IntersectionPrimitive, Vector};

use super::{Sphere, Surface};

pub fn create_box_surfaces(center: Vector, size: f32) -> Vec<Surface> {
    let front = Surface::new_vw(
        Vector::new(center.x, center.y, center.z + size / 2.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(0.0, 0.0, -1.0));
    let back = Surface::new_vw(
        Vector::new(center.x, center.y, center.z - size / 2.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(0.0, 0.0, 1.0));
    let left = Surface::new_vw(
        Vector::new(center.x - size / 2.0, center.y, center.z),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(1.0, 0.0, 0.0));
    let right = Surface::new_vw(
        Vector::new(center.x + size / 2.0, center.y, center.z),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(-1.0, 0.0, 0.0));
    let top = Surface::new_vw(
        Vector::new(center.x, center.y + size / 2.0, center.z),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(0.0, -1.0, 0.0));
    let bottom = Surface::new_vw(
        Vector::new(center.x, center.y - size / 2.0, center.z),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Some((-size / 2.0, size / 2.0)),
        Some((-size / 2.0, size / 2.0)),
        Vector::new(0.0, 1.0, 0.0));
    vec![
        front,
        back,
        left,
        right,
        top,
        bottom,
    ]    
}

pub fn create_box_primitive(center: Vector, size: f32) -> Vec<Box<dyn IntersectionPrimitive>> {
    let mut scene: Vec<Box<dyn IntersectionPrimitive>> = Vec::new();
    let mut cube = create_box_surfaces(center, size);
    for surface in cube.iter() {
        scene.push(Box::new(surface.clone()));
    }
    scene
}

pub fn create_scene() -> Vec<Box<dyn IntersectionPrimitive>> {
    let mut scene = Vec::new();
    scene.append(&mut create_box_primitive(Vector::new(0.0, 0.0, 0.0), 10.0));
    scene
}