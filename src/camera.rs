use std::path;

use crate::buffer::Buffer;
use crate::color::Color;
use crate::geometry::Line;
use crate::material::Material;
use crate::math::{RayCastHit, Vector};
use crate::scene::Scene;

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
    pub materials: Vec<Material>,
    pub buffer: Buffer,
    pub supersampling: bool,
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
            pinhole_distance: 250.0,
            materials: Vec::new(),
            buffer: Buffer::new(width as u32, height as u32),
            supersampling: false,
        }
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
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

    pub fn render_scene(&mut self, scene: &Scene, name: &str) {
        let mut path_specs = String::from(name);
        if self.perspective {
            path_specs += "_perspective";
        } else {
            path_specs += "_orthographic";
        }
        if self.supersampling {
            path_specs += "_supersampling";
        }
        path_specs += ".png";
        let new_up = self.right.cross(&self.forward);
        let mut ray = Line::new(self.position, self.forward);

        //self.buffer.colorful_checkerboard();

        let time = std::time::Instant::now();

        if !self.perspective {
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    ray.point = self.position + new_up * i as f64 + self.right * j as f64;
                    
                    let color = self.shoot_ray(&ray, scene);
                    if color.is_some() {
                        self.set_pixel_ji(j, i, color.unwrap());
                    }
                }
            }
        } else {
            let pinhole_position = self.position - self.forward * self.pinhole_distance;
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    let mut hit_colors: Vec<Color> = Vec::new();
                    //'pinhole' camera rendering
                    if self.supersampling {
                        let mut count = 0;
                        for x in -1..2 {
                            for y in -1..2 {
                                ray.point = self.position + new_up * (i as f64 + 0.25 * x as f64) + self.right * (j as f64 + 0.25 * y as f64);
                                ray.direction = Vector::from_points(pinhole_position, ray.point);

                                let color = self.shoot_ray(&ray, scene);
                                if color.is_some() {
                                    hit_colors.push(color.unwrap());
                                    count += 1;
                                }
                            }
                        }
                        let colors_len = hit_colors.len();
                        let mut pixel_color = Color::default();
                        for color in hit_colors {
                            pixel_color += color / colors_len as f32;
                        }
                        match count {
                            0 => {},
                            1..=8 => self.blend_pixel_ji(j, i, pixel_color, count as f32 / 8f32),
                            9 => self.set_pixel_ji(j, i, pixel_color),
                            _ => {println!("how??? {}", count)},
                        }
                    } else {
                        ray.point = self.position + new_up * i as f64 + self.right * j as f64;
                        ray.direction = Vector::from_points(pinhole_position, ray.point);
                        
                        let color = self.shoot_ray(&ray, scene);
                        if color.is_some() {
                            self.set_pixel_ji(j, i, color.unwrap());
                        }
                    }
                }
            }
        }
        println!("Rendering took: {}ms", time.elapsed().as_millis());
        self.buffer.save(path_specs.as_str());
    }

    pub fn add_pixel_ji(&mut self, j: i32, i: i32, color: Color) {
        self.buffer.add_to_pixel((j + self.render_width / 2) as u32, (-i + self.render_height / 2) as u32, color);
    }

    pub fn set_pixel_ji(&mut self, j: i32, i: i32, color: Color) {
        self.buffer.set_pixel((j + self.render_width / 2) as u32, (-i + self.render_height / 2) as u32, color);
    }

    pub fn blend_pixel_ji(&mut self, j: i32, i: i32, color: Color, amount: f32) {
        self.buffer.blend_pixel((j + self.render_width / 2) as u32, (-i + self.render_height / 2) as u32, color, amount);
    }

    pub fn set_camera_position(&mut self, v: &Vector) {
        self.position = *v;
    }

    pub fn get_debug_info(&self) -> String {
        self.debug.clone()
    }

    pub fn shoot_ray(&mut self, ray: &Line, scene: &Scene) -> Option<Color> {
        let mut closest_intersection = RayCastHit::new(None);
        let mut closest_distance = 0.0;
        let mut closest_material_idx = 0;

        for (i, primitive) in scene.primitives.iter().enumerate() {
            let hit = primitive.intersect(&ray);
            if hit.is_some() {
                let from_cam_to_point = hit.unwrap().0 - ray.point;
    
                if from_cam_to_point.dot(&ray.direction) >= 0.0 {
                    let intersection = hit.unwrap();
                    let distance = ray.point.distance(&intersection.0);
    
                    if closest_intersection.is_none() {
                        closest_intersection = RayCastHit::new(Some(intersection));
                        closest_material_idx = scene.material_index[i];
                        closest_distance = distance;
                    } else if distance < closest_distance {
                        closest_intersection = RayCastHit::new(Some(intersection));
                        closest_material_idx = scene.material_index[i];
                        closest_distance = distance;
                    }
                }
            }
        }

        if closest_intersection.is_some() {
            Some(self.materials[closest_material_idx].color)
        } else {
            None
        }
    }
}