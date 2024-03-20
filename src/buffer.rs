use raylib::color;

use crate::color::Color;

pub struct Buffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>, // RGB
    //pub z_buffer: Vec<f32>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            width,
            height,
            data: vec![Color::default(); (width * height) as usize],
        }
    }

    pub fn clear_color(&mut self, color: Color) {
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
                let r = (x as f32 / size as f32).floor() / x_size as f32;
                let g = (y as f32 / size as f32).floor() / y_size as f32;
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
        }
    }

    pub fn add_to_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index] += color;
        }
    }

    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color, amount: f32) {
        if x < self.width && y < self.height {
            let index = (x + (self.height - y - 1) * self.width) as usize;
            self.data[index].blend(&color, amount);
        }
    }

    pub fn save(&self, path: &str) {
        let mut img = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = self.data[(x + y * self.height) as usize].to_u8();
            *pixel = image::Rgb([color.0, color.1, color.2]);
        }
        img.save(path).unwrap();
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