use crate::math::Vector;


pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}