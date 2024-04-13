use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MaterialType {
    Phong,
    Reflective,
    Refractive,
    PBR,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub base_color: Color,
    pub specular_amount: f32,
    pub shininess: f32,
    pub material_type: MaterialType,
    pub max_bounce_depth: f32,
    pub refractive_index: f32,

    // PBR
    pub metallic: f32,
    pub roughness: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            base_color: Color::new(1.0, 1.0, 1.0),
            specular_amount: 0.0,
            shininess: 0.0,
            material_type: MaterialType::Phong,
            max_bounce_depth: 0.0,
            refractive_index: 1.0,
            metallic: 0.0,
            roughness: 0.0,
        }
    }
}

impl Material {
    pub fn new(base_color: Color, specular: f32, shininess: f32, material_type: MaterialType, max_bounce_depth: f32) -> Material {
        Material {
            base_color,
            specular_amount: specular,
            shininess,
            material_type,
            max_bounce_depth,
            ..Default::default()
        }
    }

    pub fn new_phong(base_color: Color, specular_amount: f32, shininess: f32) -> Material {
        Material {
            base_color,
            specular_amount,
            shininess,
            material_type: MaterialType::Phong,
            ..Default::default()
        }
    }

    pub fn new_reflective(base_color: Color, specular_amount: f32, shininess: f32, max_bounce_depth: f32) -> Material {
        Material {
            base_color,
            specular_amount,
            shininess,
            material_type: MaterialType::Reflective,
            max_bounce_depth,
            ..Default::default()
        }
    }

    pub fn new_refractive(base_color: Color, refractive_index: f32) -> Material {
        Material {
            base_color,
            material_type: MaterialType::Refractive,
            refractive_index,
            ..Default::default()
        }
    }

    pub fn new_pbr(albedo: Color, metallic: f32, roughness: f32) -> Material {
        Material {
            base_color: albedo,
            metallic,
            roughness,
            material_type: MaterialType::PBR,
            max_bounce_depth: 8000.0,
            ..Default::default()
        }
    }
}