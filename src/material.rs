use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MaterialType {
    Phong,
    Reflective,
    Refractive,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub base_color: Color,
    pub specular_amount: f32,
    pub shininess: f32,
    pub material_type: MaterialType,
    pub max_bounce_depth: f32,
}

impl Material {
    pub fn new(base_color: Color, specular: f32, shininess: f32, material_type: MaterialType, max_bounce_depth: f32) -> Material {
        Material {
            base_color,
            specular_amount: specular,
            shininess,
            material_type,
            max_bounce_depth,
        }
    }

    pub fn new_phong(base_color: Color, specular_amount: f32, shininess: f32) -> Material {
        Material {
            base_color,
            specular_amount,
            shininess,
            material_type: MaterialType::Phong,
            max_bounce_depth: 0.0,
        }
    }

    pub fn new_reflective(base_color: Color, specular_amount: f32, shininess: f32, max_bounce_depth: f32) -> Material {
        Material {
            base_color,
            specular_amount,
            shininess,
            material_type: MaterialType::Reflective,
            max_bounce_depth,
        }
    }
}