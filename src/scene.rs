use crate::{intersection::IntersectionPrimitive, sphere::Sphere, surface::Surface, triangle::Triangle};

pub struct Scene {
    // pub surfaces: Vec<Surface>,
    // pub spheres: Vec<Sphere>,
    // pub triangles: Vec<Triangle>,
    //pub lights: Vec<Light>,

    pub primitives: Vec<Box<dyn IntersectionPrimitive>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: Vec::new(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Box<dyn IntersectionPrimitive>) {
        self.primitives.push(primitive);
    }
}