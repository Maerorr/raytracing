use crate::math::Vector;


pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    radius_squared: f32,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Sphere {
        let radius_squared = radius * radius;
        Sphere {
            center,
            radius,
            radius_squared,
        }
    }

    pub fn get_radius_squared(&self) -> f32 {
        self.radius_squared
    }
}