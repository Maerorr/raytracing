use std::ops;
use float_cmp::{approx_eq, F64Margin};
use crate::mat4::Mat4;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub real: f64,
    pub ivec: Vector,
}

impl Quaternion {
    pub fn new(real: f64, ivec: Vector) -> Quaternion {
        Quaternion { real, ivec }
    }

    pub fn identity() -> Quaternion {
        Quaternion::new(1.0, Vector::new(0.0, 0.0, 0.0))
    }

    pub fn rotate(&mut self, angle: f64, Vector { x, y, z, w}: Vector) {
        let angle = angle * 0.5;
        let sin = angle.sin();
        let cos = angle.cos();
        let q = Quaternion::new(cos, Vector::new(x * sin, y * sin, z * sin));
        *self = *self * q;
    }

    // Using the formula P' = H(H(R, P), R*) where H is the Hamilton product and R* is the conjugate of R
    pub fn rotate_vec(&self, vec: &mut Vector) {
        let q1 = self.hamilton_product(&Quaternion::new(0.0, *vec));
        let mut self_inv = self.clone();
        self_inv.conjugate();
        let q2 = q1.hamilton_product(&self_inv);
        *vec = q2.ivec;
    }

    pub fn hamilton_product(&self, other: &Quaternion) -> Quaternion {
        let Quaternion { real: a, ivec: Vector { x: i, y: j, z: k, w: _ } } = *self;
        let Quaternion { real: b, ivec: Vector { x: l, y: m, z: n, w: _ } } = *other;
        Quaternion::new(
            a * b - i * l - j * m - k * n,
            Vector::new(
                a * l + i * b + j * n - k * m,
                a * m + j * b + k * l - i * n,
                a * n + k * b + i * m - j * l,
            ),
        )
    }

    pub fn to_mat4(&self) -> Mat4 {
        let mut mat = Mat4::new();
        let q1 = self.real;
        let q2 = self.ivec.x;
        let q3 = self.ivec.y;
        let q4 = self.ivec.z;

        mat.m[0][0] = 1.0 - 2.0 * q3 * q3 - 2.0 * q4 * q4;
        mat.m[0][1] = 2.0 * q2 * q3 - 2.0 * q1 * q4;
        mat.m[0][2] = 2.0 * q2 * q4 + 2.0 * q1 * q3;
        //mat[0][3] = 0.0;
        mat.m[1][0] = 2.0 * q2 * q3 + 2.0 * q1 * q4;
        mat.m[1][1] = 1.0 - 2.0 * q2 * q2 - 2.0 * q4 * q4;
        mat.m[1][2] = 2.0 * q3 * q4 - 2.0 * q1 * q2;
        //mat[1][3] = 0.0;
        mat.m[2][0] = 2.0 * q2 * q4 - 2.0 * q1 * q3;
        mat.m[2][1] = 2.0 * q3 * q4 + 2.0 * q1 * q2;
        mat.m[2][2] = 1.0 - 2.0 * q2 * q2 - 2.0 * q3 * q3;
        //mat[2][3] = 0.0;

        mat
    }

    pub fn inverse(&mut self) {
        let mut quat = self.clone();
        quat.conjugate();
        let divisor = self.real * self.real + self.ivec.dot(&self.ivec);
        if approx_eq!(f64, divisor, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
            print!("Warning: division by zero. Quaternion values were not altered.");
            return;
        }
        let inv = 1.0 / divisor;

        quat *= inv;

        self.real = quat.real;
        self.ivec = quat.ivec;
    }

    pub fn normalize(&mut self) {
        let inv = 1.0 / self.length();
        *self *= inv;
    }

    pub fn length(&self) -> f64 {
        (self.real * self.real + self.ivec.dot(&self.ivec)).sqrt()
    }

    pub fn conjugate(&mut self) {
        self.ivec *= -1.0;
    }

    pub fn dot(&mut self, other: &Quaternion) -> f64 {
        (self.real * other.real) + self.ivec.dot(&other.ivec)
    }

    pub fn to_string(&self) -> String {
        let out: String = format!("({:.2}, {:.2}i, {:.2}j, {:.2}k)", self.real, self.ivec.x, self.ivec.y, self.ivec.z);
        out
    }
}

// OPERATOR OVERLOADS
// + operator overload
impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            real: self.real + other.real,
            ivec: self.ivec + other.ivec,
        }
    }
}

// += operator overload
impl ops::AddAssign<Quaternion> for Quaternion {
    fn add_assign(&mut self, other: Quaternion) {
        self.real += other.real;
        self.ivec += other.ivec;
    }
}

// - operator overload
impl ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            real: self.real - other.real,
            ivec: self.ivec - other.ivec,
        }
    }
}

// -= operator overload
impl ops::SubAssign<Quaternion> for Quaternion {
    fn sub_assign(&mut self, other: Quaternion) {
        self.real -= other.real;
        self.ivec -= other.ivec;
    }
}

// * operator overload
// scalar * quaternion
impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, scalar: f64) -> Quaternion {
        Quaternion {
            real: self.real * scalar,
            ivec: self.ivec * scalar,
        }
    }
}

// *= operator overload
// quaternion *= scalar
impl ops::MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, scalar: f64) {
        self.real *= scalar;
        self.ivec *= scalar;
    }
}

// quaternion * quaternion
impl ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let real_out = self.real * other.real - self.ivec.dot(&other.ivec);
        let ivec_out = self.ivec.cross(&other.ivec) + self.ivec * other.real + other.ivec * self.real;

        Quaternion {
            real: real_out,
            ivec: ivec_out,
        }
    }
}

// *= operator overload
impl ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        let real_out = self.real * other.real - self.ivec.dot(&other.ivec);
        let ivec_out = self.ivec.cross(&other.ivec) + self.ivec * other.real + other.ivec * self.real;

        self.real = real_out;
        self.ivec = ivec_out;
    }
}

// / operator overload
impl ops::Div for Quaternion {
    type Output = Quaternion;

    fn div(self, other: Quaternion) -> Quaternion {
        let out_quat = Quaternion::new(self.real, self.ivec);
        let mut other_inv = other.clone();
        other_inv.inverse();
        let out = out_quat * other_inv;
        Quaternion {
            real: out.real,
            ivec: out.ivec,
        }
    }
}

// /= operator overload
impl ops::DivAssign for Quaternion {
    fn div_assign(&mut self, other: Quaternion) {
        let out_quat = Quaternion::new(self.real, self.ivec);
        let mut other_inv = other.clone();
        other_inv.inverse();
        let out = out_quat * other_inv;
        self.real = out.real;
        self.ivec = out.ivec;
    }
}

// / operator overload
// quaternion / scalar
impl ops::Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, scalar: f64) -> Quaternion {
        Quaternion {
            real: self.real / scalar,
            ivec: self.ivec / scalar,
        }
    }
}

// /= operator overload
impl ops::DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, scalar: f64) {
        self.real /= scalar;
        self.ivec /= scalar;
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        approx_eq!(f64, self.real, other.real, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) &&
        approx_eq!(f64, self.ivec.x, other.ivec.x, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) &&
        approx_eq!(f64, self.ivec.y, other.ivec.y, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) &&
        approx_eq!(f64, self.ivec.z, other.ivec.z, F64Margin { epsilon: f64::EPSILON, ulps: 4 })
    }
}


#[cfg(test)]
mod test {
    use crate::mat4::*;
    use crate::math::as_radians;
    use crate::vector::*;
    use super::*;

    #[test]
    fn identity_test() {
        let quat = Quaternion::identity();
        println!("\nQUATERNION IDENTITY TEST\n");
        println!("Calling Quaternion::Identity() returns an identity quaternion: {}", quat.to_string());
        assert_eq!(quat.real, 1.0);
        assert_eq!(quat.ivec.x, 0.0);
        assert_eq!(quat.ivec.y, 0.0);
        assert_eq!(quat.ivec.z, 0.0);
    }

    #[test]
    fn vector_rotation_test() {
        let mut q = Quaternion::identity();
        q.rotate(as_radians(90.0), Vector::new(0.0, 0.0, 1.0));
        let mut mat = Mat4::identity();
        mat.rotate(as_radians(90.0), Vector::new(0.0, 0.0, 1.0));
        let mut v1 = Vector::new(1.0, 1.0, 1.0);
        let mut v2 = Vector::new(1.0, 1.0, 1.0);
        println!("\nQUATERNION VECTOR ROTATION TEST\n");
        println!("Let's take a quaternion and a matrix rotated by 90deg on z axis: {},\n{}", q.to_string(), mat.to_string());
        println!("Create two identical vectors: v1 = {},\nv2 = {}", v1.to_string(), v2.to_string());
        println!("Rotate one of them with a quaternion and the second with a matrix");

        q.rotate_vec(&mut v1);
        v2 = v2 * mat;

        println!("v1 = {}, v2 = {}", v1.to_string(), v2.to_string());
        assert_eq!(v1, v2);
    }

    #[test]
    fn hamilton_prod_test() {
        let mut q1 = Quaternion::new(1.0, Vector::new(2.0, 3.0, 4.0));
        let mut q2 = Quaternion::new(2.0, Vector::new(3.0, 4.0, 5.0));

        let h = q1.hamilton_product(&q2);
        let other_h = q2.hamilton_product(&q1);

        println!("\nQUATERNION HAMILTON PRODUCT TEST\n");
        println!("Let's take two quaternions: q1 = {}, q2 = {}", q1.to_string(), q2.to_string());

        println!("Let's calculate their hamilton product.");
        println!("the output of this operation should be:\n-36 + 6i + 12j + 12k if it's H(q1, q2)\nor -36 + 8i + 8j + 14k for H(q2, q1)");
        println!("we revieved: H(q1, q2) = {}, H(q2, q1) = {}", h.to_string(), other_h.to_string());

        assert_eq!(h, Quaternion::new(-36.0, Vector::new(6.0, 12.0, 12.0)));
        assert_eq!(other_h, Quaternion::new(-36.0, Vector::new(8.0, 8.0, 14.0)));
    }

    #[test]
    // Check if the rotations are correctly applied and then converted to rotation matrix
    fn to_mat_test() {
        let mut quat = Quaternion::identity();
        quat.rotate(as_radians(90.0), Vector::new(1.0, 0.0, 0.0));
        let quat_mat = quat.to_mat4();
        println!("\nQUATERNION TO MATRIX TEST\n");
        println!("Let's take a identity quaternion and rotate it by 90deg on x axis: {}", quat.to_string());
        println!("Then let's transform it into a mat4 frms: {}", quat_mat.to_string());


        let mut other_mat = Mat4::identity();
        other_mat.rotate(as_radians(90.0), Vector::new(1.0, 0.0, 0.0));
        println!("now let's take an identity matrix and perform the same operations {}", other_mat.to_string());
        println!("they are the same, except for floating point precision errors");

        let (mut vec1, mut vec2) = (Vector::new(1.0, 1.0, 1.0), Vector::new(1.0, 1.0, 1.0));
        vec1 = vec1 * quat_mat;
        vec2 = vec2 * other_mat;
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn normalize_test() {
        let mut q = Quaternion::new(1.0, Vector::new(2.0, 3.0, 4.0));
        println!("\nQUATERNION NORMALIZE TEST\n");
        println!("Let's take a quaternion: {}", q.to_string());
        println!("Let's normalize it");
        q.normalize();
        println!("now it's 'length' should be 1: {:.4}", q.length());
        assert!(approx_eq!(f64, q.length(), 1.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }));
    }

    #[test]
    fn inverse_test() {
        let mut quat = Quaternion::new(1.0, Vector::new(2.0, 3.0, 4.0));
        let mut inverse = quat.clone();
        inverse.inverse();
        println!("\nQUATERNION INVERSE TEST\n");
        println!("Let's take a quaternion: {}", quat.to_string());
        println!("Let's calculate its inverse: {}", inverse.to_string());
        println!("Let's multiply the quaternion by its inverse");
        quat *= inverse;
        println!("The result should be an identity quaternion: {}", quat.to_string());
        assert_eq!(quat, Quaternion::new(1.0, Vector::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn conjugate_test() {
        let mut quat = Quaternion::new(1.0, Vector::new(2.0, 3.0, 4.0));
        let mut conjugate = quat.clone();
        conjugate.conjugate();
        println!("\nQUATERNION CONJUGATE TEST\n");
        println!("Let's take a quaternion: {}", quat.to_string());
        println!("Let's calculate its conjugate: {}", conjugate.to_string());
        assert_eq!(conjugate, Quaternion::new(1.0, Vector::new(-2.0, -3.0, -4.0)));
    }
}