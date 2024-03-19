use std::f64::consts::PI;

use crate::intersection::IntersectionPrimitive;
use crate::line::Line;
use crate::object::Object;
use crate::quaternion::Quaternion;
use crate::raycasthit::RayCastHit;
use crate::scene::Scene;
use crate::triangle::{self, Triangle};
use crate::{surface, Mat4};
use crate::vector::Vector;

pub struct Camera {
    pub position: Vector,
    pub forward: Vector,
    pub render_width: i32,
    pub render_height: i32,
    pub up: Vector,
    pub right: Vector,
    debug: String,
    pub backface_culling: bool,
    pub perspective: bool,
    pub pinhole_distance: f64,
}

impl Camera {
    pub fn new(position: Vector, forward: Vector, width: i32, height: i32, up: Vector) -> Camera {
        let right = forward.cross(&up);
        Camera {
            position,
            forward,
            render_width: width,
            render_height: height,
            up,
            right,
            debug: String::new(),
            backface_culling: true,
            perspective: false,
            pinhole_distance: 120.0,
        }
    }

    // OLD RENDER FUNCTION
    // pub fn render(&mut self, object: &Object) -> Vec<RayCastHit> {
    //     // THIS IS JUST TO ROTATE THE CAMERA ONCE PER RENDER WITHOUT IT SPINNING AROUND
    //     let mut l = self.line.clone();
    //     let mut point = self.line.point;
    //     point.rotate_by_quaternion(&self.rotation);
    //     l.point.rotate_by_quaternion(&self.rotation);
    //     l.direction.rotate_by_quaternion(&self.rotation);
    //     let mut up = self.up.clone();
    //     let mut right = self.right.clone();
    //     up.rotate_by_quaternion(&self.rotation);
    //     right.rotate_by_quaternion(&self.rotation);

    //     self.debug.clear();
    //     self.debug.push_str(&format!("Camera position: {}\n", l.point.to_string()));
    //     self.debug.push_str(&format!("Camera direction: {}\n", l.direction.to_string()));
    //     self.debug.push_str(&format!("Camera up: {}\n", up.to_string()));
    //     self.debug.push_str(&format!("Camera right: {}\n", right.to_string()));

    //     let mut hits: Vec<RayCastHit> = Vec::new();
    //     for i in (-self.render_height / 2)..(self.render_height / 2) {
    //         for j in (-self.render_width / 2)..(self.render_width / 2) {
    //             l.point = point + up * i as f64 + right * j as f64;
    //             let mut hit = l.intersection_object(&object, &l.point, &self.backface_culling);
    //             hit.pos_on_screen = (j, i);
    //             hits.push(hit);
    //         }
    //     }
    //     hits
    // }

    pub fn render_scene(&mut self, scene: &Scene) -> Vec<RayCastHit> {
        let mut new_up = self.right.cross(&self.forward);
        let mut ray = Line::new(self.position, self.forward);
        println!("new up: {:?}", new_up);

        let mut hits: Vec<RayCastHit> = Vec::new();
        if !self.perspective {
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;

                    ray.point = self.position + new_up * i as f64 + self.right * j as f64;
                    for primitive in &scene.primitives {
                        let hit = primitive.intersect(&ray);
                        if hit.is_some() {
                            let from_cam_to_point = hit.unwrap().0 - ray.point;
                
                            if from_cam_to_point.dot(&ray.direction) >= 0.0 {
                                let intersection = hit.unwrap();
                                let distance = ray.point.distance(&intersection.0);
                
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
    
                    closest_intersection.pos_on_screen = (j, i);
                    hits.push(closest_intersection);
                }
            }
        } else {
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;
                    //'pinhole' camera rendering

                    ray.point = self.position - self.forward * self.pinhole_distance;
                    ray.direction = Vector::from_points(ray.point, self.position + new_up * i as f64 + self.right * j as f64);

                    // let line_point = self.default.0 + up * i as f64 + right * j as f64 + front * -self.pinhole_distance;
                    // let mut line_dir = (self.default.0) - line_point;

                    for primitive in &scene.primitives {
                        let hit = primitive.intersect(&ray);
                        if hit.is_some() {
                            let from_cam_to_point = hit.unwrap().0 - ray.point;
                
                            if from_cam_to_point.dot(&ray.direction) >= 0.0 {
                                let intersection = hit.unwrap();
                                let distance = ray.point.distance(&intersection.0);
                
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

                    closest_intersection.pos_on_screen = (j, i);
                    hits.push(closest_intersection);
                }
            }
        }
        
        hits
    }

    pub fn set_camera_position(&mut self, v: &Vector) {
        self.position = *v;
    }

    pub fn get_debug_info(&self) -> String {
        self.debug.clone()
    }
}