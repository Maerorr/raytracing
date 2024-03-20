use crate::color::Color;


pub struct Material {
    pub color: Color,
}

impl Material {
    pub fn new(color: Color) -> Material {
        Material { color }
    }
}