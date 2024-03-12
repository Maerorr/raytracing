use float_cmp::{approx_eq, F64Margin};
use crate::object::Object;
use crate::point::Point;
use crate::raycasthit::RayCastHit;
use crate::surface::Surface;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point: Vector,
    pub direction: Vector,
}

impl Line {
    pub fn new(point: Vector, direction: Vector) -> Line {
        Line { point, direction }
    }

    // Returns the point of intersection if they intersect. Otherwise returns None.
    pub fn intersection(&self, other: &Line) -> Option<Vector> {
        let cross_squared = self.direction.cross(&other.direction).length_squared();
        let t1 = (other.point - self.point)
            .cross(&other.direction)
            .dot(&self.direction.clone().cross(&other.direction))
            / cross_squared;
        let t2 = (other.point - self.point)
            .cross(&self.direction)
            .dot(&self.direction.clone().cross(&other.direction))
            / cross_squared;

        let p1 = self.point + self.direction * t1;
        let p2 = other.point + other.direction * t2;

        if p1 == p2 {
            // this returns one of the values since they are the same, or very, very close to each other.
            Some(p1)
        } else {
            None
        }
    }

    pub fn angle_degrees(&self, other: &Line) -> f64 {
        let angle = self.direction.angle_degrees(&other.direction);
        angle
    }

    pub fn angle_radians(&self, other: &Line) -> f64 {
        let angle = self.direction.angle_radians(&other.direction);
        angle
    }

    // returns the expression p + tv
    pub fn point_on_line(&self, t: &f64) -> Vector {
        self.point + self.direction * *t
    }

    // Returns the point of intersection if they intersect. Otherwise returns None.
    pub fn intersection_surface(&self, surface: &Surface) -> RayCastHit {
        let parallel_check = self.direction.dot(&surface.normal);
        if approx_eq!(f64, parallel_check, 0.0, F64Margin::default()) {
            RayCastHit::new(None)
        } else {
            let t = ((surface.normal * -1.0).dot(&(self.point - surface.point)))
                / (surface.normal.dot(&self.direction));
            let intersection = self.point_on_line(&t);
            let (t, s) = surface.get_t_s_from_point(&intersection);
            let angle = self.direction.angle_radians(&surface.normal);

            // this checks if the intersection isnt outside of out bound surface (a rectangle)
            if surface.point_on_surface(&t, &s).is_none() {
                RayCastHit::new(None)
            } else {
                RayCastHit::new(Some((intersection, angle)))
            }
        }
    }

    pub fn intersection_object(&self, obj: &Object, cam_pos: &Vector, bfc: &bool) -> RayCastHit {
        let mut closest_intersection: RayCastHit = RayCastHit::new(None);
        let mut closest_distance: f64 = 0.0;

        for surface in &obj.surfaces {
            // intersection with each surface
            let intersection = self.intersection_surface(&surface);
            // if we have a hit
            if intersection.is_some() {

                let from_cam_to_point = intersection.unwrap().0 - *cam_pos;

                if from_cam_to_point.dot(&self.direction) >= 0.0 {
                    let intersection = intersection.unwrap();
                    let distance = (*cam_pos).distance(&intersection.0);

                    if closest_intersection.is_none() {
                        closest_intersection = RayCastHit::new(Some(intersection));
                        closest_distance = distance;
                    } else if distance < closest_distance {
                        closest_intersection = RayCastHit::new(Some(intersection));
                        closest_distance = distance;
                    }
                }
            }
        }
        if closest_intersection.is_some() {
            if *bfc {
                if closest_intersection.angle().cos() < 0.0 {
                    //println!("camera pos: {}, hit: {}, angle: {}", cam_pos.to_string(), closest_intersection.unwrap().0.to_string(), closest_intersection.angle());
                    closest_intersection = RayCastHit::new(None);
                }
            }
        }
        closest_intersection
    }
}

