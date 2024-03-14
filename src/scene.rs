use crate::{sphere::Sphere, surface::Surface, triangle::Triangle};

pub struct Scene {
    pub surfaces: Vec<Surface>,
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
    //pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            surfaces: Vec::new(),
            spheres: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}