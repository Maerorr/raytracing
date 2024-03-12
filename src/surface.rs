use crate::point::Point;
use crate::quaternion::Quaternion;
use crate::vector::Vector;

// Surface is defined by a point and a normal vector
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub point: Vector,
    pub v: Option<Vector>,
    pub max_v: Option<(f64, f64)>,
    pub w: Option<Vector>,
    pub max_w: Option<(f64, f64)>,
    pub normal: Vector,
}

impl Surface {
    // create surface from point and a normal vector
    pub fn new_normal(point: Vector, normal: Vector) -> Surface {
        Surface { point, v: None, max_v: None, w: None, max_w: None, normal}
    }

    // create surface from point and two vectors
    pub fn new_vw(point: Vector, v: Vector, w: Vector, max_v: (f64, f64), max_w: (f64, f64), normal: Vector) -> Surface {
        // let mut normal = v.cross(&w);
        // normal.normalize();
        Surface { point, v: Some(v), w: Some(w), max_v: Some(max_v), max_w: Some(max_w), normal}
    }

    // return the distance from the surface to a point
    pub fn distance(&self, point: &Point) -> f64 {
        let v = point.to_vector() - self.point;
        v.dot(&self.normal)
    }

    pub fn get_t_s_from_point(&self, point: &Vector) -> (f64, f64) {
        let v = *point - self.point;
        let t = v.dot(&self.v.unwrap()) / self.v.unwrap().length_squared();
        let s = v.dot(&self.w.unwrap()) / self.w.unwrap().length_squared();
        (t, s)
    }

    // return the point on the surface closest to the given point
    pub fn closest_point(&self, point: &Point) -> Point {
        let v = point.to_vector() - self.point;
        let d = v.dot(&self.normal);
        let v = self.normal * d;
        Point::from_vector(&(point.to_vector() - v))
    }

    // returns expression Q + tv + sw. Returns None if surface was not defined with v and w. Or if t or s are outside the bounds of the surface.
    pub fn point_on_surface(&self, t: &f64, s: &f64) -> Option<Vector> {
        if self.v.is_some() && self.w.is_some() {
            return if *t > self.max_v.unwrap().1 || *t < self.max_v.unwrap().0 || *s > self.max_w.unwrap().1 || *s < self.max_w.unwrap().0 {
                None
            } else {
                let v = self.v.unwrap() * *t;
                let w = self.w.unwrap() * *s;
                Some(self.point + v + w)
            }
        }
        None
    }

    pub fn rotate(&mut self, q: &Quaternion) {
        if self.v.is_some() && self.w.is_some() {
            let mut new_v = self.v.unwrap().clone();
            new_v.rotate_by_quaternion(&q);
            let mut new_w = self.w.unwrap().clone();
            new_w.rotate_by_quaternion(&q);
            self.v = Some(new_v);
            self.w = Some(new_w);
        }
        self.point.rotate_by_quaternion(&q);
        self.normal.rotate_by_quaternion(&q);
    }

    pub fn scale(&mut self, s: &f64) {
        self.point.x *= s;
        self.point.y *= s;
        self.point.z *= s;

        let (mut v1, mut v2) = self.max_v.unwrap();
        v1 *= s;
        v2 *= s;
        self.max_v = Some((v1, v2));
        let (mut v1, mut v2) = self.max_w.unwrap();
        v1 *= s;
        v2 *= s;
        self.max_w = Some((v1, v2));

    }
}