use crate::{light::Light, math::intersection::IntersectionPrimitive};

pub struct Scene {
    pub primitives: Vec<Box<dyn IntersectionPrimitive + Send + Sync>>,
    pub material_index: Vec<usize>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: Vec::new(),
            material_index: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Box<dyn IntersectionPrimitive + Send + Sync>, material_idx: usize) {
        self.primitives.push(primitive);
        self.material_index.push(material_idx);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_lights(&mut self, lights: Vec<Light>) {
        for light in lights {
            self.lights.push(light);
        }
    }
}