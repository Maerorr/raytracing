use std::collections::HashMap;
use std::f32::consts::PI;
use std::path;
use std::sync::Arc;
use std::thread::Thread;

use float_cmp::F32Margin;
use image::Pixel;

use crate::buffer::Buffer;
use crate::color::Color;
use crate::geometry::Line;
use crate::light::{LightCalculationData, LightType};
use crate::material::{self, Material, MaterialType};
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
    pub pinhole_distance: f32,
    pub materials: Vec<Material>,
    pub buffer: Buffer,
    pub antialias_debug_buffer: Buffer,
    pub aa_type: AntiAliasingType,
    pub max_bounces: i32,
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
            max_bounces: 4,
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

        self.buffer.clear_color(Color::black());

        let time = std::time::Instant::now();

        if self.aa_type == AntiAliasingType::Supersampling4x {
            // Supersampling means: Render at twice the resolution and then shrink by two, interpolating the colors
            self.render_width *= 2;
            self.render_height *= 2;
            self.buffer = Buffer::new(self.render_width as u32, self.render_height as u32);
        }

        if !self.perspective {
            for i in (-self.render_height / 2 + 1)..(self.render_height / 2) {
                for j in (-self.render_width / 2)..(self.render_width / 2) {
                    ray.point = self.position + new_up * i as f32 + self.right * j as f32;
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
                    ray.point = self.position + new_up * i as f32 + self.right * j as f32;
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
                                        new_up * (i as f32 + 0.25 * offset_x as f32) + 
                                        self.right * (j as f32 + 0.25 * offset_y as f32);

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
                let count = hit_colors.len() + 1;
                //average_color /= count as f32;
                //println!("avg color: {:?}", average_color);
                // self.set_pixel_ji(j, i, average_color);
                
                match count {
                    0 => {},
                    1..=8 => {
                        let bg_color = self.buffer.clear_color;
                        let color = average_color + bg_color * (9 - count) as f32;
                        self.set_pixel_ji(j, i, color / 9.0);
                    }
                    9 => self.set_pixel_ji(j, i, average_color / count as f32),
                    _ => {println!("how??? {}", count)},
                }
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

    pub fn get_pixel_ji(&self, j: i32, i: i32) -> Color {
        self.buffer.get_pixel((j + self.render_width / 2) as u32, (-i + self.render_height / 2) as u32)
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
                        closest_intersection = hit.clone();
                        closest_material_idx = scene.material_index[i];
                        closest_distance = distance;
                    } else if distance < closest_distance {
                        closest_intersection = hit.clone();
                        closest_material_idx = scene.material_index[i];
                        closest_distance = distance;
                    }
                }
            }
        }

        if closest_intersection.is_some() {
            let mut color = Color::black();
            let intersection = closest_intersection.unwrap().0;
            let normal = closest_intersection.normal.unwrap();
            let material = &self.materials[closest_material_idx];

            let lighting_data = LightCalculationData {
                point: intersection,
                normal,
                view_dir: ray.direction,
                base_color: material.base_color,
                shininess: material.shininess,
                specular_amount: material.specular_amount,
            };

            for light in scene.lights.iter() {
                if light.light_type == LightType::Ambient {
                    let light_color = light.calculate_lighting(&lighting_data);
                    color += light_color;
                    continue;
                } else {
                    // shot ray into the light
                    let light_dir = (light.position - intersection)._normalize();
                    let line_pos = intersection + light_dir * 0.01;
                    let light_ray = Line::new(line_pos, light_dir);
                    let distance = intersection.distance(&light.position);
                    let shadowed = shoot_ray_into_light(&light_ray, scene, distance);

                    if !shadowed {
                        let light_color = light.calculate_lighting(&lighting_data);
                        color += light_color;
                    }
                }
            }
            
            Some(color)
        } else {
            None
        }
    }

    pub fn render_scene_multithreaded(&mut self, scene: Scene, name: &str) {
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

        self.buffer.clear_color(Color::black());

        let time = std::time::Instant::now();

        if self.aa_type == AntiAliasingType::Supersampling4x {
            // Supersampling means: Render at twice the resolution and then shrink by two, interpolating the colors
            self.render_width *= 2;
            self.render_height *= 2;
            self.buffer = Buffer::new(self.render_width as u32, self.render_height as u32);
        }
        // create two threads
        let mut handles = vec![];

        // Arc is Rust's read-only shared pointer
        let scene_arc = Arc::new(scene);

        let mut thread_data_vec: Vec<ThreadRenderDara> = Vec::new();
        // for 2 threads, time decreases by about half, for 4 threads, time decreases by about 1/4
        // but futher the time doesn't decrease linearly. 10x decrease is seen for 16 thread.
        // after than the time only increases
        let thread_nums = 16;
        println!("rendering with {} threads", thread_nums);

        for i in 0..thread_nums {
            let min_i = -self.render_height / 2 + (self.render_height / thread_nums) * i;
            let max_i = -self.render_height / 2 + (self.render_height / thread_nums) * (i + 1);
            let min_j = -self.render_width / 2;
            let max_j = self.render_width / 2;
            let thread_data = ThreadRenderDara {
                min_i,
                max_i,
                min_j,
                max_j,
                position: self.position,
                up: new_up,
                right: self.right,
                forward: self.forward,
                perspective: self.perspective,
                aa_type: self.aa_type,
                pinhole_distance: self.pinhole_distance,
                materials: self.materials.clone(),
                scene: scene_arc.clone(),
                sky_color: self.buffer.clear_color,
                max_bounces: self.max_bounces,
            };
            thread_data_vec.push(thread_data);
        }

        for thread_data in thread_data_vec {
            let handle = std::thread::spawn(move || {
                render_thread(thread_data)
            });
            handles.push(handle);
        }

        let mut output_pixels: Vec<Option<Color>> = Vec::new();

        for handle in handles {
            let output = handle.join().unwrap();
            for  color in output {
                output_pixels.push(color);
            }
        }

        for (i, color) in output_pixels.iter().enumerate() {
            if color.is_some() {
                self.buffer.write_pixel_by_idx(i, color.unwrap());
            } else {
                println!("no color for pixel {}", i);
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
                                        new_up * (i as f32 + 0.25 * offset_x as f32) + 
                                        self.right * (j as f32 + 0.25 * offset_y as f32);

                        if self.perspective {
                            ray.direction = Vector::from_points(pinhole_position, ray.point);
                        }
                        let color = self.shoot_ray(&ray, &scene_arc);
                        if color.is_some() {
                            hit_colors.push(color.unwrap());
                        }
                    }
                }

                let mut average_color = self.buffer.get_pixel(*x as u32, *y as u32);
                for color in hit_colors.iter() {
                    average_color += *color;
                }
                let count = hit_colors.len() + 1;
                
                match count {
                    0 => {},
                    1..=8 => {
                        let bg_color = self.buffer.clear_color;
                        let color = average_color + bg_color * (9 - count) as f32;
                        self.set_pixel_ji(j, i, color / 9.0);
                    }
                    9 => self.set_pixel_ji(j, i, average_color / count as f32),
                    _ => {println!("how??? {}", count)},
                }
            }
        }

        println!("Rendering took: {}ms", time.elapsed().as_millis());

        if self.aa_type == AntiAliasingType::Supersampling4x {
            self.buffer.shrink_by_two();
        }
        self.buffer.save(path_specs.as_str());
    }
}

pub struct ThreadRenderDara {
    pub min_i: i32,
    pub max_i: i32,
    pub min_j: i32,
    pub max_j: i32,
    pub position: Vector,
    pub up: Vector,
    pub right: Vector,
    pub forward: Vector,
    pub perspective: bool,
    pub aa_type: AntiAliasingType,
    pub pinhole_distance: f32,
    pub materials: Vec<Material>,
    pub scene: Arc<Scene>,
    pub sky_color: Color,
    pub max_bounces: i32,
}

pub fn render_thread(data: ThreadRenderDara) -> Vec<Option<Color>>{
    let mut output: Vec<Option<Color>> = Vec::new();
    if !data.perspective {
        let pinhole_position = data.position - data.forward * data.pinhole_distance;
        let mut ray = Line::new(data.position, data.forward);
        for i in data.min_i..data.max_i {
            for j in data.min_j..data.max_j {
                ray.point = data.position + data.up * i as f32 + data.right * j as f32;
                if data.aa_type == AntiAliasingType::Supersampling4x {
                    ray.point /= 2.0;
                }
                let color = p_shoot_ray(&ray, pinhole_position, &data.scene, &data.materials, data.max_bounces, data.sky_color);
                output.push(color);
            }
        }
    } else {
        let pinhole_position = data.position - data.forward * data.pinhole_distance;
        let mut ray = Line::new(data.position, data.forward);
        for (ci, i) in (data.min_i..data.max_i).enumerate() {
            for (cj, j) in (data.min_j..data.max_j).enumerate() {
                //'pinhole' camera rendering
                ray.point = data.position + data.up * i as f32 + data.right * j as f32;
                if data.aa_type == AntiAliasingType::Supersampling4x {
                    ray.point /= 2.0;
                }

                ray.direction = Vector::from_points(pinhole_position, ray.point)._normalize();

                let mut color = p_shoot_ray(&ray, pinhole_position, &data.scene, &data.materials, data.max_bounces, data.sky_color);
                output.push(color);
            }
        }
    }

    output
}

pub fn p_shoot_ray(ray: &Line, pinhole_position: Vector, scene: &Scene, materials: &Vec<Material>, max_bounces: i32, sky_color: Color) -> Option<Color> {
    if max_bounces == -1 {
        return Some(sky_color);
    }
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
                    closest_intersection = hit.clone();
                    closest_material_idx = scene.material_index[i];
                    closest_distance = distance;
                } else if distance < closest_distance {
                    closest_intersection = hit.clone();
                    closest_material_idx = scene.material_index[i];
                    closest_distance = distance;
                }
            }
        }
    }

    if closest_intersection.is_some() {
        let mut color = Color::black();
        let intersection = closest_intersection.unwrap().0;
        let normal = closest_intersection.normal.unwrap();
        let material = &materials[closest_material_idx];

        let lighting_data = LightCalculationData {
            point: intersection,
            normal,
            view_dir: ray.direction,
            base_color: material.base_color,
            shininess: material.shininess,
            specular_amount: material.specular_amount,
        };

        match material.material_type {
            MaterialType::Phong => {
                for light in scene.lights.iter() {
                    if light.light_type == LightType::Ambient {
                        let light_color = light.calculate_lighting(&lighting_data);
                        color += light_color;
                        continue;
                    } else {
                        // shot ray into the light
                        let light_dir = (light.position - intersection)._normalize();
                        let line_pos = intersection + light_dir * 0.01;
                        let light_ray = Line::new(line_pos, light_dir);
                        let distance = intersection.distance(&light.position);
                        let shadowed = shoot_ray_into_light(&light_ray, scene, distance);
        
                        if !shadowed {
                            let light_color = light.calculate_lighting(&lighting_data);
                            color += light_color;
                            color._clamp01();
                        }
                    }
                }
            },
            MaterialType::Reflective => {
                let reflected_dir = ray.direction.reflect(&normal);
                let reflected_ray_start = intersection + reflected_dir * 0.1;
                let reflected_ray = Line::new(reflected_ray_start, reflected_dir);
                let reflected_color = p_shoot_ray(&reflected_ray, pinhole_position, scene, materials, max_bounces - 1, sky_color);
                if reflected_color.is_some() {
                    color = reflected_color.unwrap();
                }
            },
            MaterialType::Refractive => {
                let refracted_dir = ray.direction.refract(&normal, material.refractive_index);
                let refracted_ray_start = intersection + refracted_dir * 0.1;
                let refracted_ray = Line::new(refracted_ray_start, refracted_dir);
                let refracted_color = p_shoot_ray(&refracted_ray, pinhole_position, scene, materials, max_bounces - 1, sky_color);
                if refracted_color.is_some() {
                    color = refracted_color.unwrap();
                }
            },
            MaterialType::PBR => {
                let albedo = material.base_color;
                let roughness = material.roughness;
                let metalic = material.metallic;
                let anisotropy = material.anisotropy;
                let ior = 1.5;
                let f0 = Vector::new(0.04, 0.04, 0.04);
                let albedo_vec = albedo.to_vector();
                let f0 = Vector::lerp(&f0, &albedo_vec, metalic);

                let mut lo = Color::black();
                for light in scene.lights.iter() {
                    if light.light_type == LightType::Point {
                        let l = (light.position - intersection)._normalize();
                        let v = (pinhole_position - intersection)._normalize();
                        let h = (v + l)._normalize();
                        let cos_theta = normal.dot(&l).max(0.0);

                        let distance = intersection.distance(&light.position);
                        let attenuation = 1.0 / (light.attenuation.0 + light.attenuation.1 * distance + light.attenuation.2 * distance * distance);
                        let radiance = light.color * attenuation;

                        let mut D = normal_distribution(&normal, &h, roughness);
                        if anisotropy > 0.0 {
                            let tangent = (Vector::new(0.0, 0.0, 1.0).cross(&normal))._normalize();
                            let binormal = normal.cross(&tangent);
                            let an = ggx_anisotropic(&h, &normal, &tangent, &binormal, 0.02, 0.1);
                            D = anisotropy * an + ((1.0 - anisotropy) * D);
                        }

                        let G = geometry_smith(normal, v, l, roughness);

                        let F = fresnel_schlick(cos_theta, Color::from(f0));

                        let numerator = F * D * G;
                        let denom = 4.0 * normal.dot(&v).max(0.0) * normal.dot(&l).max(0.0) + 0.001;
                        let specular = numerator / denom;
                        let ks = F;
                        let kd = Color::white() - ks;
                        let kd = kd * (1.0 - metalic);
                       // println!("ks: {:?}, kd: {:?}", ks, kd);
                        //println!("kd: {}, albedo: {}, spec: {}, radiance: {}, cos_t: {}", kd.to_string(), albedo.to_string(), specular, radiance.to_string(), cos_theta);
                        lo += (kd * albedo + specular) * radiance * cos_theta;
                    }
                }
                let ambient = albedo * 0.003;
                let mut pixel_color = lo + ambient;
                pixel_color = pixel_color / (pixel_color + Color::white());
                pixel_color.gamma_correction(2.2);
                color += pixel_color;
            }
        }

        Some(color)
    } else {
        None
    }
}

pub fn normal_distribution(n: &Vector, h: &Vector, roughness: f32) -> f32 {
    let a2 = (roughness * roughness).powi(2);
    let ndoth: f32 = n.dot(h);
    let ndoth2 = ndoth * ndoth;
    let num = a2;
    let mut denom = ndoth2 * (a2 - 1.0) + 1.0;
    denom = 4.0 * denom * denom;
    num / denom
}

// GGX Anisotropic [5] from: https://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
pub fn ggx_anisotropic(h: &Vector, n: &Vector, x: &Vector, y: &Vector, ax: f32, ay: f32, ) -> f32 {
    let first = 1.0 / (PI * ax * ay);
    let ndoth2 = n.dot(h) * n.dot(h);
    let xh = (x.dot(h)).powi(2) / ax.powi(2);
    let yh = (y.dot(h)).powi(2) / ay.powi(2);
    let second = 1.0 / (xh + yh + ndoth2).powi(2);
    first * second
}

pub fn shoot_ray_into_light(ray: &Line, scene: &Scene, max_distance: f32) -> bool {
    for primitive in scene.primitives.iter() {
        let hit = primitive.intersect(&ray);
        if hit.is_some() {
            let intersection = hit.unwrap().0;
            let distance = ray.point.distance(&intersection);
            if distance < max_distance {
                return true;
            }
        }
    }
    return false
}


pub fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    NdotV / (NdotV * (1.0 - k) + k)
}

pub fn geometry_smith(N: Vector, V: Vector, L: Vector, roughness: f32) -> f32 {
    let ndotv = N.dot(&V).max(0.0);
    let ndotl = N.dot(&L).max(0.0);
    let ggx2 = geometry_schlick_ggx(ndotv, roughness);
    let ggx1 = geometry_schlick_ggx(ndotl, roughness);
    ggx1 * ggx2
}

pub fn fresnel_schlick(cos_theta: f32, f0: Color) -> Color {
    f0 + (Color::white() - f0) * ((1.0 - cos_theta).clamp(0.0, 1.0)).powi(5)
}

//return None;
// let mut f0 = Color::new(0.04, 0.04, 0.04);
// f0.blend(&material.base_color, material.metallic);
// let albedo;
// let metallic;
// let roughness;
// if material.textured && closest_intersection.uv.is_some() {
//     let uv = closest_intersection.uv.unwrap();
//     let u = uv.0;
//     let v = uv.1;
//     let tex_size = material.albedo_map.dimensions();
//     let color = material.albedo_map.get_pixel((u * (tex_size.0 - 1) as f32) as u32, (v * (tex_size.1 - 1) as f32) as u32).to_rgb();
//     let tex_size = material.roughness_map.dimensions();
//     let rough_sampled = material.roughness_map.get_pixel((u * (tex_size.0 - 1) as f32) as u32, (v * (tex_size.1 - 1) as f32) as u32).to_rgb();
//     let tex_size = material.metallic_map.dimensions();
//     let metallic_sampled = material.metallic_map.get_pixel((u * (tex_size.0 - 1) as f32) as u32, (v * (tex_size.1 - 1) as f32) as u32).to_rgb();
//     albedo = Color::new(color[0] as f32 / 255.0, color[1] as f32 / 255.0, color[2] as f32 / 255.0);
//     metallic = metallic_sampled[0] as f32 / 255.0;
//     roughness = rough_sampled[0] as f32 / 255.0;
// } else {
//     albedo = material.base_color;
//     metallic = material.metallic;
//     roughness = material.roughness;
// }

// let mut lo = Color::black();
// let V = (pinhole_position - intersection)._normalize();
// for light in scene.lights.iter() {
//     if light.light_type == LightType::Point {
//         let light_dir = (light.position - intersection)._normalize();
//         let line_pos = intersection + light_dir * 0.01;
//         let light_ray = Line::new(line_pos, light_dir);
//         let distance = intersection.distance(&light.position);
//         let shadowed = shoot_ray_into_light(&light_ray, scene, distance);

//         if shadowed {
//             continue;
//         }

//         let L = (light.position - intersection)._normalize();
//         let H = (V + L)._normalize();
//         let distance_for_att = intersection.distance(&light.position) / 100.0;
//         let att = 1.0 / (1.0 + distance_for_att * distance_for_att);
//         let radiance = light.color * att;

//         let NDF: f32 = distribution_ggx(normal, H, roughness);
//         let G = geometry_smith(normal, V, L, roughness);
//         let F = fresnel_schlick(V.dot(&H), f0);

//         let numerator = F.to_vector() * NDF * G;
//         let denominator = 4.0 * normal.dot(&V).max(0.0) * normal.dot(&L).max(0.0) + 0.0001;
//         let specular = numerator / denominator;

//         let kS = F;
//         let mut kD = Color::white() - kS;
//         kD *= 1.0 - metallic;
//         let ndotl = normal.dot(&L).max(0.0);
//         lo += Color::new(NDF, NDF, NDF);//(kD * albedo / std::f32::consts::PI + Color::from(specular)) * radiance * ndotl;
//     }
// }
// let ambient = Color::black();//Color::white() * 0.01 * albedo;
// let mut pixel_color = ambient + lo;
// //println!("{}", pixel_color.to_string());
// // pixel_color = pixel_color / (pixel_color + Color::white());
// // pixel_color.gamma_correction(2.2);
// color = pixel_color;