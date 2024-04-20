use float_cmp::{approx_eq};

use crate::{geometry::{Line, Sphere, Surface, Triangle}};

use super::{RayCastHit, Vector};


pub trait IntersectionPrimitive {
    fn intersect(&self, ray: &Line) -> RayCastHit;
}

impl IntersectionPrimitive for Surface {
    // implementation modified from: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < 0.0001 {
            return RayCastHit::new(None);
        }

        let p0l0 = self.point - ray.point;
        let t = p0l0.dot(&self.normal) / denom;
        let intersection = ray.point_on_line(&t);
        let angle = ray.direction.angle_radians(&self.normal);

        if t > 0.0 {
            let ts = self.get_t_s_from_point(&intersection); // this works correct
        
            if self.point_on_surface(&ts.0, &ts.1).is_none() {
                return RayCastHit::new(None);
            }
            let distance = (intersection - ray.point).length();

            if self.max_v.is_some() && self.max_w.is_some() {
                let u = (ts.0 - self.max_v.unwrap().0) / (self.max_v.unwrap().1 - self.max_v.unwrap().0);
                let v = (ts.1 - self.max_w.unwrap().0) / (self.max_w.unwrap().1 - self.max_w.unwrap().0);
                let uv = (u, v);
                //println!("uv: {:.3?}", uv);
                return RayCastHit::new(Some((intersection, angle))).with_normal(self.normal).with_distance(distance).with_uv(uv)
            }

            RayCastHit::new(Some((intersection, angle))).with_normal(self.normal).with_distance(distance)//.with_uv(uv)
        } else {
            RayCastHit::new(None)
        }
    }
}

impl IntersectionPrimitive for Sphere {
    // geometric solution from: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let r2 = self.get_radius_squared();
        let l = self.center - ray.point;
        let tca = l.dot(&ray.direction);
        if tca < 0.0 {
            return RayCastHit::new(None);
        }
        let d2 = l.dot(&l) - tca * tca;
        if d2 > r2 {
            return RayCastHit::new(None);
        }
        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;
        //println!("t0: {}, t1: {}", t0, t1);
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return RayCastHit::new(None);
            }
        }
        let intersection = ray.point_on_line(&t0);
        let mut normal = intersection - self.center;
        normal.normalize();

        let angle = ray.direction.angle_radians(&normal);
        let distance = (intersection - ray.point).length();
        RayCastHit::new(Some((intersection, angle))).with_normal(normal).with_distance(distance)
    }
}

impl IntersectionPrimitive for Triangle {
    fn intersect(&self, ray: &Line) -> RayCastHit {
        // MOLLER-TRUMBORE METHOD
        let v0v1 = self.vertices[1] - self.vertices[0];
        let v0v2 = self.vertices[2] - self.vertices[0];
        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);
        //println!("det: {}", det);
        if det.abs() < 0.0001 {
            return RayCastHit::new(None);
        }
        let inv_det = 1.0 / det;
        let tvec = ray.point - self.vertices[0];
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            //println!("u is out of bounds");
            return RayCastHit::new(None);
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            //println!("v is out of bounds");
            return RayCastHit::new(None);
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if t > 0.00001 {
            let intersection = ray.point_on_line(&t);
            let angle = ray.direction.angle_radians(&self.normal);
            let distance = (intersection - ray.point).length();
            RayCastHit::new(Some((intersection, angle))).with_normal(self.normal).with_distance(distance)
        } else {
            //println!("t is out of bounds");
            RayCastHit::new(None)
        }
    }

}
