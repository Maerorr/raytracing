use core::panic;

use raylib::color;

use crate::color::Color;

pub struct Buffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>, // RGB

    pub clear_color: Color,
    //pub z_buffer: Vec<f32>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            width,
            height,
            data: vec![Color::default(); (width * height) as usize],
            clear_color: Color::default(),
        }
    }

    pub fn clear_color(&mut self, color: Color) {
        self.clear_color = color;
        for pixel in self.data.iter_mut() {
            *pixel = color;
        }
    }

    pub fn colorful_checkerboard(&mut self) {
        let size = 64;
        let x_size = self.width / size;
        let y_size = self.height / size;
        for y in 0..self.height {
            for x in 0..self.width {
                // make reds rise from left to right
                let r = (x as f64 / size as f64).floor() / x_size as f64;
                let g = (y as f64 / size as f64).floor() / y_size as f64;
                let b = 0.0;
                let color = Color::new(r, g, b);
                self.set_pixel(x, y, color);
            }
        }
    }

    // this function also flips y so it is displayed correctly
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index] = color;
        } else {
            println!("Tried to set pixel out of bounds: ({}, {})", x, y);
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index]
        } else {
            panic!("Tried to get pixel out of bounds: ({}, {})", x, y);
        }
    }

    pub fn add_to_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index] += color;
        }
    }

    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color, amount: f64) {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index].blend(&color, amount);
        }
    }

    pub fn save(&self, path: &str) {
        let mut img = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = (x + (self.height - y - 1) * self.width) as usize;
            let color = self.data[idx].to_u8();
            *pixel = image::Rgb([color.0, color.1, color.2]);
        }
        img.save(path).unwrap();
    }

    pub fn shrink_by_two(&mut self) {
        let mut new_data = Vec::new();
        // take 2x2 pixels and average them
        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                let mut color = Color::default();
                for i in 0..2 {
                    for j in 0..2 {
                        let index = (x + j + (y + i) * self.width) as usize;
                        color += self.data[index];
                    }
                }
                color /= 4.0;
                new_data.push(color);
            }
        }
        self.width /= 2;
        self.height /= 2;
        self.data = new_data;
    }

    pub fn into_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
    
        for color in self.data.iter() {
            let c = color.to_u8();
            vec.push(c.0);
            vec.push(c.1);
            vec.push(c.2);
        }
        vec
    }
}