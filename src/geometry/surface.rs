use crate::math::{Vector, Quaternion};

// Surface is defined by a point and a normal vector
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub point: Vector,
    pub v: Option<Vector>,
    pub max_v: Option<(f32, f32)>,
    pub w: Option<Vector>,
    pub max_w: Option<(f32, f32)>,
    pub normal: Vector,
}

impl Surface {
    // create surface from point and a normal vector
    pub fn new_normal(point: Vector, normal: Vector) -> Surface {
        // calculate v and w vectors
        let mut v = Vector::new(1.0, 0.0, 0.0);
        if normal.x.abs() > 0.9 {
            v = Vector::new(0.0, 1.0, 0.0);
        }
        let mut w = normal.cross(&v);
        v = normal.cross(&w);
        v.normalize();
        w.normalize();

        Surface { point, v: Some(v) , max_v: None, w: Some(w), max_w: None, normal}
    }

    // create surface from point and two vectors
    pub fn new_vw(point: Vector, v: Vector, w: Vector, max_v: Option<(f32, f32)>, max_w: Option<(f32, f32)>, normal: Vector) -> Surface {
        // let mut normal = v.cross(&w);
        // normal.normalize();
        Surface { point, v: Some(v), w: Some(w), max_v: max_v, max_w: max_w, normal}
    }

    // return the distance from the surface to a point
    pub fn distance(&self, point: &Vector) -> f32 {
        let v = *point - self.point;
        v.dot(&self.normal)
    }

    pub fn get_t_s_from_point(&self, point: &Vector) -> (f32, f32) {
        let v = *point - self.point;
        let t = v.dot(&self.v.unwrap()) / self.v.unwrap().length_squared();
        let s = v.dot(&self.w.unwrap()) / self.w.unwrap().length_squared();
        (t, s)
    }

    // return the point on the surface closest to the given point
    pub fn closest_point(&self, point: &Vector) -> Vector {
        let v = *point - self.point;
        let d = v.dot(&self.normal);
        let v = self.normal * d;
        //Point::from_vector(&(point.to_vector() - v))
        *point - v
    }

    // returns expression Q + tv + sw. Returns None if surface was not defined with v and w. Or if t or s are outside the bounds of the surface.
    pub fn point_on_surface(&self, t: &f32, s: &f32) -> Option<Vector> {
        if self.v.is_some() && self.w.is_some() {
            if self.max_v.is_none() || self.max_w.is_none() {
                //ignore bounds
                let v = self.v.unwrap() * *t;
                let w = self.w.unwrap() * *s;
                return Some(self.point + v + w);
            }
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

    pub fn scale(&mut self, s: &f32) {
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

    pub fn translate(&mut self, v: &Vector) {
        self.point += *v;
    }
}