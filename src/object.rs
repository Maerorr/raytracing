use crate::Quaternion;
use crate::surface::Surface;

pub struct Object {
    pub surfaces: Vec<Surface>,
}

impl Object {
    pub fn new(surfaces: Vec<Surface>) -> Object {
        Object { surfaces }
    }

    pub fn rotate(&mut self, q: &Quaternion) {
        for surface in &mut self.surfaces {
            surface.rotate(q);
        }
    }

    pub fn scale(&mut self, s: &f64) {
        for surface in &mut self.surfaces {
            surface.scale(s);
        }
    }
}