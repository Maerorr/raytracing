use std::{fmt::{self, Display, Formatter}, ops::{Add, AddAssign, Div, DivAssign, Mul, Sub, SubAssign}};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        // if self.r > 1.0 || self.g > 1.0 || self.b > 1.0 || self.r < 0.0 || self.g < 0.0 || self.b < 0.0 {
        //     panic!("Color values must be between 0.0 and 1.0");
        // }
        let r = (self.r * 255.0).clamp(0.0, 255.0) as u8;
        let g = (self.g * 255.0).clamp(0.0, 255.0) as u8;
        let b = (self.b * 255.0).clamp(0.0, 255.0) as u8;
        (r, g, b)
    }

    pub fn blend(&mut self, other: &Color, amount: f32) {
        self.r = self.r * (1.0 - amount) + other.r * amount;
        self.g = self.g * (1.0 - amount) + other.g * amount;
        self.b = self.b * (1.0 - amount) + other.b * amount;
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

// implement add, sub, mul, div for Color
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, other: Color) -> Color {
        Color {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        if other == 0.0 {
            return self;
        }
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        if other == 0.0 {
            return;
        }
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Color: ({}, {}, {})", self.r, self.g, self.b)
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        let this = self.to_u8();
        let other = other.to_u8();
        this.0 == other.0 && this.1 == other.1 && this.2 == other.2
    }
}