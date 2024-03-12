use crate::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    // basic constructor
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn from_vector(v: &Vector) -> Point {
        Point { x: v.x, y: v.y, z: v.z }
    }

    // distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    // return a vector from (0,0,0) to this point
    pub fn to_vector(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
}