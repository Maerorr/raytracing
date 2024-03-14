use crate::point::Point;
use crate::vector::Vector;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub color: Vector,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Vector) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }
}