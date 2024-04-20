use std::{mem::swap, ops};
// used for comparing floats
use float_cmp::{approx_eq, F32Margin};
use crate::color::Color;

use super::{Mat4, Quaternion};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    
    /// Constructor
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z , w: 1.0}
    }

    pub fn from_num(num: f32) -> Vector {
        Vector { x: num, y: num, z: num, w: 1.0 }
    }

    /// create a vector that points from one point to another
    pub fn from_points(start: Vector, end: Vector) -> Vector {
        let direction = (end - start)._normalize();
        Vector {
            x: direction.x,
            y: direction.y,
            z: direction.z,
            w: 1.0,
        }
    }

    /// dot product, multiplication of all components
    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// cross product, result is a perpendicular vector
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: self.w,
        }
    }
    
    /// Returns the angle between two vectors in **radians**
    pub fn angle_radians(&self, other: &Vector) -> f32 {
        // get the dot product
        let dot = self.dot(other);
        // calculate lengths of both vectors
        let len1 = self.length();
        let len2 = other.length();
        // calculate the angle
        let cos = dot / (len1 * len2);
        // return the angle in radians
        cos.acos()
    }

    pub fn angle_degrees(&self, other: &Vector) -> f32 {
        self.angle_radians(other) * 180.0 / std::f32::consts::PI
    }

    /// Returns the length of a vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Normalizes a vector, which means it makes it's length equal to 1
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn _normalize(&self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w,
        }
    }

    // rotates self by a given quaternion
    // algorithm https://gamedev.stackexchange.com/questions/28395/rotating-vector3-by-a-quaternion
    pub fn rotate_by_quaternion(&mut self, q: &Quaternion) {
        // Extract the vector part of the quaternion
        let u = Vector::new(q.ivec.x, q.ivec.y, q.ivec.z);
        // Extract the scalar part of the quaternion
        let s = q.real;
        // Do the math
        // vprime = 2.0f * dot(u, v) * u
        //     + (s*s - dot(u, u)) * v
        //     + 2.0f * s * cross(u, v);
        let mut vprime = u * 2.0 * u.dot(self) +
                            *self * (s * s - u.dot(&u)) +
                            u.cross(self) * 2.0 * s;
        // Copy the result back into self
        self.x = vprime.x;
        self.y = vprime.y;
        self.z = vprime.z;
    }

    pub fn distance(&self, other: &Vector) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2)).sqrt()
    }

    /// Converts a vector to a string and returns it
    pub fn to_string(&self) -> String {
        let out: String = format!("[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z);
        out
    }

    pub fn multiply_by_matrix(&self, mat: &Mat4) -> Vector {
        let mut out = Vector::new(0.0, 0.0, 0.0);

        println!("{}", mat.to_string());
        out.x = self.x * mat.m[0][0] + self.y * mat.m[0][1] + self.z * mat.m[0][2] + self.w * mat.m[0][3];
        out.y = self.x * mat.m[1][0] + self.y * mat.m[1][1] + self.z * mat.m[1][2] + self.w * mat.m[1][3];
        out.z = self.x * mat.m[2][0] + self.y * mat.m[2][1] + self.z * mat.m[2][2] + self.w * mat.m[2][3];
        out.w = self.x * mat.m[3][0] + self.y * mat.m[3][1] + self.z * mat.m[3][2] + self.w * mat.m[3][3];

        if out.w != 1.0 {
            out.x /= out.w;
            out.y /= out.w;
            out.z /= out.w;
        }

        out
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        let dot = self.dot(normal);
        let mut out = *self - *normal * 2.0 * dot;
        out.normalize();
        out
    }

    pub fn refract(&self, n: &Vector, eta: f32) -> Vector {
        let i = self._normalize();
        let mut n_dot_i = n.dot(self);
        let eta_air = 1.0;
        let eta_material = eta;
        let eta;
        let mut n = *n;
        // if the cos is less than zero, we're leaving the refractive material and going to air
        if n_dot_i < 0.0 {
            n_dot_i = -n_dot_i;
            n = -n;
            eta = eta_air / eta_material;
        } else {
            eta = eta_material / eta_air;
        }

        // code like in GLSL refract function
        let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k < 0.0 {
            Vector::new(0.0, 0.0, 0.0)
        } else {
            i * eta - n * (eta * n_dot_i - k.sqrt())
        }
    }

    pub fn random(min: f32, max: f32) -> Vector {
        let x = min + (max - min) * rand::random::<f32>();
        let y = min + (max - min) * rand::random::<f32>();
        let z = min + (max - min) * rand::random::<f32>();
        Vector::new(x, y, z)
    }

    pub fn lerp(a: &Vector, b: &Vector, t: f32) -> Vector {
        *a * (1.0 - t) + *b * t
    }
}

impl From<Color> for Vector {
    fn from(color: Color) -> Vector {
        Vector::new(color.r, color.g, color.b)
    }
}

// + operator overload
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w,
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// - operator overload
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w,
        }
    }
}

impl ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// * operator overload
// scalar multiply ([vector] * scalar)
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w,
        }
    }
}

impl ops::Mul<Mat4> for Vector {
    type Output = Vector;

    fn mul(self, mat: Mat4) -> Vector {
        let mut out = Vector::new(0.0, 0.0, 0.0);
        out.x = self.x * mat.m[0][0] + self.y * mat.m[0][1] + self.z * mat.m[0][2] + mat.m[0][3];
        out.y = self.x * mat.m[1][0] + self.y * mat.m[1][1] + self.z * mat.m[1][2] + mat.m[1][3];
        out.z = self.x * mat.m[2][0] + self.y * mat.m[2][1] + self.z * mat.m[2][2] + mat.m[2][3];
        out.w = self.x * mat.m[3][0] + self.y * mat.m[3][1] + self.z * mat.m[3][2] + mat.m[3][3];
        out
    }
}

impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// / operator overload
// vector scalar division ([vector] / scalar)
// in case of division by zero, return the original vector
impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        if scalar == 0.0 {
            println!("Warning: division by zero. Vector values were not altered.");
            return self;
        } else
        {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
                w: self.w,
            }
        }
    }
}

impl ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, scalar: f32) {
        if scalar == 0.0 {
            print!("Warning: division by zero. Vector values were not altered.");
        } else
        {
            self.x /= scalar;
            self.y /= scalar;
            self.z /= scalar;
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        // use approx_eq!
        approx_eq!(f32, self.x, other.x, F32Margin { epsilon: f32::EPSILON, ulps: 4 }) &&
        approx_eq!(f32, self.y, other.y, F32Margin { epsilon: f32::EPSILON, ulps: 4 }) &&
        approx_eq!(f32, self.z, other.z, F32Margin { epsilon: f32::EPSILON, ulps: 4 })
        //self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use crate::math::as_radians;
    use super::*;

    #[test]
    fn sub_add_test() {
        // test, przemienność dodawania
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        let v4 = v2 + v1;
        assert_eq!(v3, v4);
        let v5 = v3 - v2;
        assert_eq!(v1, v5);
    }

    #[test]
    fn mul_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        let v3 = v2 * 0.5;
        assert_eq!(v1, v3);
    }

    #[test]
    fn from_points_test() {
        let start = Vector::new(0.0, 0.0, 0.0);
        let end = Vector::new(0.0, 0.0, 1.0);
        let v1 = Vector::from_points(start, end);
        // normalized result
        let v2 = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn dot_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn cross_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&v2);
        let v4 = Vector::new(-3.0, 6.0, -3.0);
        assert_eq!(v3, v4);
    }

    #[test]
    fn angle_test() {
        let v1 = Vector::new(0.0, 3.0, 0.0);
        let v2 = Vector::new(5.0, 5.0, 0.0);
        let mut angle = v1.angle_radians(&v2);
        // simple trick to round decimal places
        // 0.123499 * 10000.0 = 1234.99 -> 1234 -> 1234.0 / 10000.0 = 0.1234
        angle = (angle*10000.0).round() / 10000.0;
        assert_eq!(angle, 0.7854);
    }

    #[test]
    fn normalize_test() {
        let mut vector = Vector::new(15.0, 12.0, -15.0);
        vector.normalize();
        // because of float precision issues, we need to round the values.
        let vec_string = format!("{:.5}", vector.length());
        assert_eq!(vec_string, "1.00000");
    }

    #[test]
    fn length_test() {
        let vector = Vector::new(1.0, 2.0, -3.0);
        let vec_string = format!("{:.4}", vector.length());
        assert_eq!(vec_string, "3.7417");
    }

    #[test]
    fn division_by_zero_test() {
        let x = 1.0;
        let vector = Vector::new(x, 2.0, -3.0);
        let zero_vec = vector / 0.0;
        assert_eq!(zero_vec.x, x);
    }

    #[test]
    fn ops_assign_test() {
        let vec1 = Vector::new(1.0, 2.0, 3.0);
        let vec2 = Vector::new(4.0, 5.0, 6.0);
        let mut vec3 = vec1;
        vec3 += vec2;
        assert_eq!(vec3, Vector::new(5.0, 7.0, 9.0));
        vec3 -= vec2;
        assert_eq!(vec3, vec1);
        vec3 *= 2.0;
        assert_eq!(vec3, vec1 * 2.0);
        vec3 /= 2.0;
        assert_eq!(vec3, vec1);
        vec3 /= 0.0;
        assert_eq!(vec3, vec1);
    }

    #[test]
    fn matrix_mul_test() {
        let mut vec = Vector::new(1.0, 2.0, 3.0);
        let mut mat = Mat4::identity();
        mat.scale(Vector::new(2.0, 2.0, 2.0));
        let vec2 = vec * mat;
        assert_eq!(vec2, Vector::new(2.0, 4.0, 6.0));
        vec = Vector::new(1.0, 0.0, 0.0);
        let mut mat = Mat4::identity();
        mat.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));
        let vec2 = vec * mat;
        assert_eq!(vec2, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn quaternion_rotation_test() {
        let mut vec = Vector::new(1.0, 0.0, 0.0);
        let mut quat = Quaternion::identity();
        quat.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));
        vec.rotate_by_quaternion(&quat);
        assert_eq!(vec, Vector::new(0.0, 0.0, -1.0));
    }
}