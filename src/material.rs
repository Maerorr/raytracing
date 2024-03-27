use crate::color::Color;


pub struct Material {
    pub base_color: Color,
    pub specular_amount: f32,
    pub shininess: f32
}

impl Material {
    pub fn new(base_color: Color, specular: f32, shininess: f32) -> Material {
        Material {
            base_color,
            specular_amount: specular,
            shininess,
        }
    }
}