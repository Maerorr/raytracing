use image::ImageBuffer;

use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MaterialType {
    Phong,
    Reflective,
    Refractive,
    PBR,
}

#[derive(Debug, Clone)]
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
    pub ior: f32,
    pub anisotropy: f32,

    pub textured: bool,
    pub albedo_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub metallic_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub roughness_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
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
            ior: 1.3,
            anisotropy: 0.0,
            textured: false,
            albedo_map: ImageBuffer::new(1, 1),
            metallic_map: ImageBuffer::new(1, 1),
            roughness_map: ImageBuffer::new(1, 1),
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

    pub fn new_pbr(albedo: Color, metallic: f32, roughness: f32, ior: f32, anisotropy: f32) -> Material {
        let roughness = roughness.clamp(0.01, 0.99);
        let metallic = metallic.clamp(0.01, 0.99);
        Material {
            base_color: albedo,
            metallic: metallic,
            roughness: roughness,
            ior: ior,
            anisotropy: anisotropy,
            material_type: MaterialType::PBR,
            max_bounce_depth: 8000.0,
            ..Default::default()
        }
    }

    pub fn new_textured_pbr(albedo: ImageBuffer<image::Rgb<u8>, Vec<u8>>, metal: ImageBuffer<image::Rgb<u8>, Vec<u8>>, roughness: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Material {
        Material {
            material_type: MaterialType::PBR,
            max_bounce_depth: 8000.0,
            textured: true,
            roughness: 1.0,
            albedo_map: albedo,
            metallic_map: metal,
            roughness_map: roughness,
            ..Default::default()
        }
    }
}