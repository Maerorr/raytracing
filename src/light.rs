use crate::{color::Color, math::Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightCalculationData {
    pub point: Vector,
    pub normal: Vector,
    pub view_dir: Vector,
    pub base_color: Color,
    pub shininess: f32,
    pub specular_amount: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    Point,
    Ambient,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vector,
    pub color: Color,
    pub strength: f32,
                   // const, lin, quad
    pub attenuation: (f32, f32, f32),
}

impl Light {
    pub fn new(light_type: LightType, position: Vector, color: Color, strength: f32, attenuation: (f32, f32, f32)) -> Light {
        Light {
            light_type,
            position,
            color,
            strength,
            attenuation,
        }
    }

    pub fn new_ambient(color: Color, strength: f32) -> Light {
        Light {
            light_type: LightType::Ambient,
            position: Vector::new(0.0, 0.0, 0.0),
            color,
            strength,
            attenuation: (0.0, 0.0, 0.0),
        }
    }

    pub fn new_point(position: Vector, color: Color, attenuation: (f32, f32, f32)) -> Light {
        Light {
            light_type: LightType::Point,
            position,
            color,
            strength: 1.0,
            attenuation,
        }
    }

    pub fn calculate_lighting(&self, data: &LightCalculationData) -> Color {
        match self.light_type {
            LightType::Ambient => data.base_color * (self.color * self.strength),
            LightType::Point => {
                let mut col = Color::black();
                // diffuse
                let light_dir = (self.position - data.point)._normalize();
                let diff = data.normal.dot(&light_dir).max(0.0);
                let att = 1.0 / 
                    (   // attenuation
                        self.attenuation.0 + // constant
                        self.attenuation.1 * (self.position - data.point).length() + // linear
                        self.attenuation.2 * (self.position - data.point).length_squared() // quadratic
                    );
                let diffuse_color = self.color * (diff * att);
                // specular
                
                let view_dir = -data.view_dir;
                let reflect_dir = (-light_dir).reflect(&data.normal);
                let spec = view_dir.dot(&reflect_dir).max(0.0).powf(data.shininess);
                let specular_color = self.color * (spec * data.specular_amount * att);

                col += (diffuse_color + specular_color) * data.base_color;
                col
            }
        }
    }
}


pub struct RectangleAreaLight {
    pub position: Vector,
    pub color: Color,
    pub attenuation: (f32, f32, f32),
    pub lights: Vec<Light>,
}

impl RectangleAreaLight {
    pub fn new(
        position: Vector, 
        color: Color, 
        attenuation: (f32, f32, f32), 
        v: Vector, w: Vector, 
        v_size: f32, w_size: f32, 
        light_density: f32) -> RectangleAreaLight {
        let mut lights = Vec::new();
        let v_step = v_size / light_density;
        let w_step = w_size / light_density;
        for i in 0..(light_density as i32) {
            for j in 0..(light_density as i32) {
                let light_pos = position + v * (i as f32) * v_step + w * (j as f32) * w_step;
                lights.push(Light::new_point(light_pos, color, attenuation));
            }
        }
        println!("created {} lights", lights.len());
        RectangleAreaLight {
            position,
            color,
            attenuation,
            lights,
        }
    }

    pub fn get_lights(&self) -> Vec<Light> {
        self.lights.clone()
    }
}
