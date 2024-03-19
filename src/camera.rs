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
    pub line: Line,
    pub render_width: i32,
    pub render_height: i32,
    pub up: Vector,
    pub right: Vector,
    default: (Vector, Vector),
    rotation: Quaternion,
    debug: String,
    pub backface_culling: bool,
    pub perspective: bool,
    pub pinhole_distance: f64,
}

impl Camera {
    pub fn new(camera_position: Vector, camera_look_direction: Vector, width: i32, height: i32, up: Vector, right: Vector) -> Camera {
        Camera {
            line: Line::new(camera_position, camera_look_direction),
            render_width: width,
            render_height: height,
            default: (camera_position, camera_look_direction),
            up,
            right,
            rotation: Quaternion::identity(),
            debug: String::new(),
            backface_culling: false,
            perspective: false,
            pinhole_distance: 120.0,
        }
    }

    pub fn render(&mut self, object: &Object) -> Vec<RayCastHit> {
        // THIS IS JUST TO ROTATE THE CAMERA ONCE PER RENDER WITHOUT IT SPINNING AROUND
        let mut l = self.line.clone();
        let mut point = self.line.point;
        point.rotate_by_quaternion(&self.rotation);
        l.point.rotate_by_quaternion(&self.rotation);
        l.direction.rotate_by_quaternion(&self.rotation);
        let mut up = self.up.clone();
        let mut right = self.right.clone();
        up.rotate_by_quaternion(&self.rotation);
        right.rotate_by_quaternion(&self.rotation);

        self.debug.clear();
        self.debug.push_str(&format!("Camera position: {}\n", l.point.to_string()));
        self.debug.push_str(&format!("Camera direction: {}\n", l.direction.to_string()));
        self.debug.push_str(&format!("Camera up: {}\n", up.to_string()));
        self.debug.push_str(&format!("Camera right: {}\n", right.to_string()));

        let mut hits: Vec<RayCastHit> = Vec::new();
        for i in (-self.render_height / 2)..(self.render_height / 2) {
            for j in (-self.render_width / 2)..(self.render_width / 2) {
                l.point = point + up * i as f64 + right * j as f64;
                let mut hit = l.intersection_object(&object, &l.point, &self.backface_culling);
                hit.pos_on_screen = (j, i);
                hits.push(hit);
            }
        }
        hits
    }

    pub fn render_scene(&mut self, scene: &Scene) -> Vec<RayCastHit> {
        let mut l = self.line.clone();
        let mut point = self.line.point;
        point.rotate_by_quaternion(&self.rotation);
        l.point.rotate_by_quaternion(&self.rotation);
        l.direction.rotate_by_quaternion(&self.rotation);
        let mut up = self.up.clone();
        let mut right = self.right.clone();
        up.rotate_by_quaternion(&self.rotation);
        right.rotate_by_quaternion(&self.rotation);
        let mut hits: Vec<RayCastHit> = Vec::new();
        if !self.perspective {
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;
    
                    l.point = point + up * i as f64 + right * j as f64;
                    for primitive in &scene.primitives {
                        let hit = primitive.intersect(&l);
                        if hit.is_some() {
                            let from_cam_to_point = hit.unwrap().0 - l.point;
                
                            if from_cam_to_point.dot(&l.direction) >= 0.0 {
                                let intersection = hit.unwrap();
                                let distance = l.point.distance(&intersection.0);
                
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
            // render with perspective using projection matrix
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;
                    //implement a 'pinhole' camera
                    let front = self.right.cross(&self.up);

                    let line_point = self.default.0 + up * i as f64 + right * j as f64 + front * -self.pinhole_distance;
                    let mut line_dir = (self.default.0) - line_point;
                    line_dir.normalize();
                    let line = Line::new(line_point, line_dir);

                    for primitive in &scene.primitives {
                        let hit = primitive.intersect(&line);
                        if hit.is_some() {
                            let from_cam_to_point = hit.unwrap().0 - line.point;
                
                            if from_cam_to_point.dot(&line.direction) >= 0.0 {
                                let intersection = hit.unwrap();
                                let distance = line.point.distance(&intersection.0);
                
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
        self.line.point = *v;
    }

    pub fn set_camera_rotation(&mut self, q: &Quaternion) {
        self.rotation = *q;
    }

    pub fn set_as_default(&mut self) {
        self.default = (self.line.point, self.line.direction);
    }

    pub fn default(&mut self) {
        self.line.point = self.default.0;
        self.line.direction = self.default.1;
    }

    pub fn get_debug_info(&self) -> String {
        self.debug.clone()
    }
}