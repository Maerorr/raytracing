use crate::color::Color;


#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub base_color: Color,
    pub specular_amount: f64,
    pub shininess: f64
}

impl Material {
    pub fn new(base_color: Color, specular: f64, shininess: f64) -> Material {
        Material {
            base_color,
            specular_amount: specular,
            shininess,
        }
    }
}