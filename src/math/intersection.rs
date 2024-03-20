use float_cmp::{approx_eq, F64Margin};

use crate::{geometry::{Line, Sphere, Surface, Triangle}};

use super::RayCastHit;


pub trait IntersectionPrimitive {
    fn intersect(&self, ray: &Line) -> RayCastHit;
}

impl IntersectionPrimitive for Surface {
    // implementation modified from: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let surface = self;

        let denom = self.normal.dot(&ray.direction);
        if approx_eq!(f64, denom, 0.0, F64Margin::default()) {
            return RayCastHit::new(None);
        }

        let p0l0 = self.point - ray.point;
        let t = p0l0.dot(&self.normal) / denom;
        let intersection = ray.point_on_line(&t);
        let angle = ray.direction.angle_radians(&self.normal);

        if t > 0.0 {
            let ts = self.get_t_s_from_point(&intersection);
            if self.point_on_surface(&ts.0, &ts.1).is_none() {
                return RayCastHit::new(None);
            }
            RayCastHit::new(Some((intersection, angle)))
            
        } else {
            RayCastHit::new(None)
        }
    }
}

impl IntersectionPrimitive for Sphere {
    // geometric solution from: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let r2 = self.radius * self.radius;

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
        RayCastHit::new(Some((intersection, angle)))
    }
}

impl IntersectionPrimitive for Triangle {
    fn intersect(&self, ray: &Line) -> RayCastHit {
        // MOLLER-TRUMBORE METHOD
        //println!("ray: {}", ray.to_string());
        let v0v1 = self.vertices[1] - self.vertices[0];
        //println!("v0v1: {}", v0v1.to_string());
        let v0v2 = self.vertices[2] - self.vertices[0];
        //println!("v0v2: {}", v0v2.to_string());
        let pvec = ray.direction.cross(&v0v2);
        //println!("pvec: {}", pvec.to_string());
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
            RayCastHit::new(Some((intersection, angle)))
        } else {
            //println!("t is out of bounds");
            RayCastHit::new(None)
        }
    }
}
