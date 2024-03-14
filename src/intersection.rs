use float_cmp::{approx_eq, F64Margin};

use crate::{
    line::Line,
    raycasthit::RayCastHit,
    sphere::Sphere,
    surface::{self, Surface},
    triangle::Triangle,
};

pub trait Intersection {
    fn intersect(&self, ray: &Line) -> RayCastHit;
}

impl Intersection for Surface {
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let surface = self;
        let parallel_check = ray.direction.dot(&surface.normal);
        if approx_eq!(f64, parallel_check, 0.0, F64Margin::default()) {
            RayCastHit::new(None)
        } else {
            let t = ((surface.normal * -1.0).dot(&(ray.point - surface.point)))
                / (surface.normal.dot(&ray.direction));
            let intersection = ray.point_on_line(&t);
            let (t, s) = surface.get_t_s_from_point(&intersection);
            let angle = ray.direction.angle_radians(&surface.normal);

            if surface.point_on_surface(&t, &s).is_none() {
                RayCastHit::new(None)
            } else {
                RayCastHit::new(Some((intersection, angle)))
            }
        }
    }
}

impl Intersection for Sphere {
    fn intersect(&self, ray: &Line) -> RayCastHit {
        let l = self.center - ray.point;
        let tca = l.dot(&ray.direction);
        if tca < 0.0 {
            return RayCastHit::new(None);
        }
        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return RayCastHit::new(None);
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return RayCastHit::new(None);
        }
        let t = if t0 < t1 { t0 } else { t1 };
        let intersection = ray.point_on_line(&t);
        let mut normal = intersection - self.center;
        normal.normalize();
        let angle = ray.direction.angle_radians(&normal);
        RayCastHit::new(Some((intersection, angle)))
    }
}

impl Intersection for Triangle {
    fn intersect(&self, ray: &Line) -> RayCastHit {
        // GENERIC METHOD
        // let ndot_ray_dir = self.normal.dot(&ray.direction);
        // if (ndot_ray_dir).abs() < 0.0001 {
        //     return RayCastHit::new(None); // no intersection, parallel
        // }

        // let d = -self.normal.dot(&self.vertices[0]);
        // let t = -(self.normal.dot(&ray.point) + d) / ndot_ray_dir;
        // if t < 0.0 {
        //     return RayCastHit::new(None); // no intersection, behind the ray
        // }

        // let p = ray.point_on_line(&t);

        // // EDGE 0
        // let edge0 = self.vertices[1] - self.vertices[0];
        // let vp0 = p - self.vertices[0];
        // let c = edge0.cross(&vp0);
        // if self.normal.dot(&c) < 0.0 {
        //     return RayCastHit::new(None); // no intersection
        // }

        // // EDGE 1
        // let edge1 = self.vertices[2] - self.vertices[1];
        // let vp1 = p - self.vertices[1];
        // let c = edge1.cross(&vp1);
        // if self.normal.dot(&c) < 0.0 {
        //     return RayCastHit::new(None); // no intersection
        // }

        // // EDGE 2
        // let edge2 = self.vertices[0] - self.vertices[2];
        // let vp2 = p - self.vertices[2];
        // let c = edge2.cross(&vp2);
        // if self.normal.dot(&c) < 0.0 {
        //     return RayCastHit::new(None); // no intersection
        // }

        // let angle = ray.direction.angle_radians(&self.normal);
        // RayCastHit::new(Some((p, angle)))

        // MOLLER-TRUMBORE METHOD
        let v0v1 = self.vertices[1] - self.vertices[0];
        let v0v2 = self.vertices[2] - self.vertices[0];
        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);
        if det.abs() < 0.0001 {
            return RayCastHit::new(None);
        }
        let inv_det = 1.0 / det;
        let tvec = ray.point - self.vertices[0];
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return RayCastHit::new(None);
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return RayCastHit::new(None);
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if t > 0.00001 {
            let intersection = ray.point_on_line(&t);
            let angle = ray.direction.angle_radians(&self.normal);
            RayCastHit::new(Some((intersection, angle)))
        } else {
            RayCastHit::new(None)
        }
        
        // let av = self.vertices[0];
        // let bv = self.vertices[1];
        // let cv = self.vertices[2];
        // let edge1 = bv - av;
        // let edge2 = cv - av;
        // let h = ray.direction.cross(&edge2);
        // let a = edge1.dot(&h);
        // if approx_eq!(f64, a, 0.0, F64Margin::default()) {
        //     return RayCastHit::new(None);
        // }
        // let f = 1.0 / a;
        // let s = ray.point - av;
        // let u = f * s.dot(&h);
        // if u < 0.0 || u > 1.0 {
        //     return RayCastHit::new(None);
        // }
        // let q = s.cross(&edge1);
        // let v = f * ray.direction.dot(&q);
        // if v < 0.0 || u + v > 1.0 {
        //     return RayCastHit::new(None);
        // }
        // let t = f * edge2.dot(&q);
        // if t > 0.00001 {
        //     let intersection = ray.point_on_line(&t);
        //     let normal = edge1.cross(&edge2);
        //     let angle = ray.direction.angle_radians(&normal);
        //     RayCastHit::new(Some((intersection, angle)))
        // } else {
        //     RayCastHit::new(None)
        // }
    }
}
