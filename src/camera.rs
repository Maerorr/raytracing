use std::f64::consts::PI;

use crate::intersection::Intersection;
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
    pub fov: f64,
    pub far: f64,
    pub near: f64,
    pub projection_matrix: Mat4
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
            fov: 90.0,
            far: 1000.0,
            near: 0.05,
            projection_matrix: Mat4::identity()
        }
    }

    pub fn set_perspective(&mut self, fov: f64, far: f64, near: f64) {
        self.perspective = true;
        // create projection matrix
        let scale = 1.0 / (fov * 0.5 * PI / 180.0).tan();
        let mut m = Mat4::identity();
        m.m[0][0] = scale;
        m.m[1][1] = scale;
        m.m[2][2] = -far / (far - near);
        m.m[3][2] = -far * near / (far - near);
        m.m[3][2] = -1.0;
        m.m[3][3] = 0.0;
        //m.transpose();
        println!("Projection matrix: \n{}", m.to_string());
        self.projection_matrix = m;
        self.fov = fov;
        self.far = far;
        self.near = near;
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

            for tri in &scene.triangles {
                println!("{}", tri.to_string());
            }

            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;
    
                    l.point = point + up * i as f64 + right * j as f64;
                    for surface in &scene.surfaces {
                        let mut hit = surface.intersect(&l);
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
                    for sphere in &scene.spheres {
                        let mut hit = sphere.intersect(&l);
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
                    for triangle in &scene.triangles {
                        let mut hit = triangle.intersect(&l);
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
            let mut triangles: Vec<Triangle> = Vec::new();
            for triangle in scene.triangles.to_vec() {
                let mut t = triangle.clone();
                for v in t.vertices.iter_mut() {
                    *v = *v * self.projection_matrix;
                    if v.w != 1.0 {
                        *v = *v / v.w;
                    }
                }
                triangles.push(t);
            }


            // render with perspective using projection matrix
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut closest_intersection = RayCastHit::new(None);
                    let mut closest_distance = 0.0;

                    l.point = point + up * i as f64 + right * j as f64;
                    for triangle in &triangles {                      
                        let mut hit = triangle.intersect(&l);
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