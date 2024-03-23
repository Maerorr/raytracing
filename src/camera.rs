use std::collections::HashMap;
use std::path;

use crate::buffer::Buffer;
use crate::color::Color;
use crate::geometry::Line;
use crate::material::Material;
use crate::math::{RayCastHit, Vector};
use crate::scene::Scene;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum AntiAliasingType {
    None,
    Supersampling4x,
    AdaptiveX,
    AdaptiveO,
}

impl AntiAliasingType {
    pub fn to_string(&self) -> &str {
        match self {
            AntiAliasingType::None => "None",
            AntiAliasingType::Supersampling4x => "Supersampling4x",
            AntiAliasingType::AdaptiveX => "AdaptiveX",
            AntiAliasingType::AdaptiveO => "AdaptiveO",
        }
    }
}

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
    pub antialias_debug_buffer: Buffer,
    pub aa_type: AntiAliasingType,
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
            antialias_debug_buffer: Buffer::new(width as u32, height as u32),
            aa_type: AntiAliasingType::None,
        }
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn render_scene(&mut self, scene: &Scene, name: &str) {
        let mut path_specs = String::from(name);
        if self.perspective {
            path_specs += "_perspective_";
        } else {
            path_specs += "_orthographic_";
        }
        path_specs += self.aa_type.to_string();
        path_specs += ".png";
        let new_up = self.right.cross(&self.forward);
        let mut ray = Line::new(self.position, self.forward);

        //self.buffer.colorful_checkerboard();

        let time = std::time::Instant::now();

        if self.aa_type == AntiAliasingType::Supersampling4x {
            // Supersampling means: Render at twice the resolution and then shrink by two, interpolating the colors
            self.render_width *= 2;
            self.render_height *= 2;
            self.buffer = Buffer::new(self.render_width as u32, self.render_height as u32);
        }

        if !self.perspective {
            for i in (-self.render_height / 2)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    ray.point = self.position + new_up * i as f64 + self.right * j as f64;
                    if self.aa_type == AntiAliasingType::Supersampling4x {
                        ray.point /= 2.0;
                    }
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
                    //'pinhole' camera rendering
                    ray.point = self.position + new_up * i as f64 + self.right * j as f64;
                    if self.aa_type == AntiAliasingType::Supersampling4x {
                        ray.point /= 2.0;
                    }

                    ray.direction = Vector::from_points(pinhole_position, ray.point);
                    
                    let color = self.shoot_ray(&ray, scene);
                    if color.is_some() {
                        self.set_pixel_ji(j, i, color.unwrap());
                    }
                }
            }
        }

        if self.aa_type == AntiAliasingType::AdaptiveX || self.aa_type == AntiAliasingType::AdaptiveO {
            // pixels (x, y) marked for additional rays.
            let mut marked_for_antialiasing: HashMap<(i32, i32), bool> = HashMap::new();

            for x in 1..(self.render_width - 1) {
                for y in 1..(self.render_height - 1) {
                    let center_color = self.buffer.get_pixel(x as u32, y as u32);
                    // get all 8 pixels surrounding the pixel
                    let mut surrounding_pixels: Vec<(Color, i32, i32)> = Vec::new();
                    if self.aa_type == AntiAliasingType::AdaptiveO {
                        for xx in -1..2 {
                            for yy in -1..2 {
                                if xx == 0 && yy == 0 {
                                    continue;
                                }
                                surrounding_pixels.push((self.buffer.get_pixel((x + xx) as u32, (y + yy) as u32), x + xx, y + yy));
                            }
                        }
                    } else {
                        surrounding_pixels.push((self.buffer.get_pixel((x - 1) as u32, y as u32), x - 1, y));
                        surrounding_pixels.push((self.buffer.get_pixel((x + 1) as u32, y as u32), x + 1, y));
                        surrounding_pixels.push((self.buffer.get_pixel(x as u32, (y - 1) as u32), x, y - 1));
                        surrounding_pixels.push((self.buffer.get_pixel(x as u32, (y + 1) as u32), x, y + 1));
                    }                    

                    for px in surrounding_pixels.iter() {
                        if px.0 != center_color {
                            marked_for_antialiasing.insert((x, y), true);
                            marked_for_antialiasing.insert((px.1, px.2), true);
                            self.antialias_debug_buffer.set_pixel(px.1 as u32, px.2 as u32, Color::new(1.0, 0.0, 0.0));
                        }
                    }
                }
            }

            for (x, y) in marked_for_antialiasing.keys() {
                let mut hit_colors: Vec<Color> = Vec::new();
                let (j, i) = self.xy_to_ji(*x, *y);
                let pinhole_position = self.position - self.forward * self.pinhole_distance;

                for offset_x in -1..2 {
                    for offset_y in -1..2 {
                        if offset_x == 0 && offset_y == 0 {
                            continue;
                        }
                        ray.point = self.position + 
                                        new_up * (i as f64 + 0.25 * offset_x as f64) + 
                                        self.right * (j as f64 + 0.25 * offset_y as f64);

                        if self.perspective {
                            ray.direction = Vector::from_points(pinhole_position, ray.point);
                        }
                        let color = self.shoot_ray(&ray, scene);
                        if color.is_some() {
                            hit_colors.push(color.unwrap());
                        }
                    }
                }

                let mut average_color = self.buffer.get_pixel(*x as u32, *y as u32);
                for color in hit_colors.iter() {
                    average_color += *color;
                }
                average_color /= 9.0;
                self.set_pixel_ji(j, i, average_color);
            }
        }

        println!("Rendering took: {}ms", time.elapsed().as_millis());

        if self.aa_type == AntiAliasingType::Supersampling4x {
            self.buffer.shrink_by_two();
        }
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

    pub fn ji_to_xy(&self, j: i32, i: i32) -> (i32, i32) {
        (j + self.render_width / 2, -i + self.render_height / 2)
    }

    pub fn xy_to_ji(&self, x: i32, y: i32) -> (i32, i32) {
        (x - self.render_width / 2, self.render_height / 2 - y)
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