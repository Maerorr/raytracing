use crate::{color::Color, math::Vector};

pub struct LightCalculationData {
    pub point: Vector,
    pub normal: Vector,
    pub view_dir: Vector,
    pub base_color: Color,
    pub shininess: f32,
    pub specular_amount: f32,
}

pub enum LightType {
    Point,
    Ambient,
}

pub struct Light {
    pub light_type: LightType,
    pub position: Vector,
    pub color: Color,
                   // const, lin, quad
    pub attenuation: (f32, f32, f32),
}

impl Light {
    pub fn new(light_type: LightType, position: Vector, color: Color, attenuation: (f32, f32, f32)) -> Light {
        Light {
            light_type,
            position,
            color,
            attenuation,
        }
    }

    pub fn new_ambient(color: Color) -> Light {
        Light {
            light_type: LightType::Ambient,
            position: Vector::new(0.0, 0.0, 0.0),
            color,
            attenuation: (0.0, 0.0, 0.0),
        }
    }

    pub fn new_point(position: Vector, color: Color, attenuation: (f32, f32, f32)) -> Light {
        Light {
            light_type: LightType::Point,
            position,
            color,
            attenuation,
        }
    }

    pub fn calculate_lighting(&self, data: &LightCalculationData) -> Color {
        match self.light_type {
            LightType::Ambient => data.base_color * self.color,
            LightType::Point => {
                Color::new(0.0, 0.0, 0.0)
            }
        }
    }
}
