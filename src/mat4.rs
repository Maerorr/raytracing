use std::{ops};
use crate::{vector::*, math::*};

// used for partial_eq
use float_cmp::{approx_eq, F64Margin};

// 4x4 matrix of f64, row-major
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub m: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 {
            m: [[0.0; 4]; 4],
        }
    }

    // create an identity matrix
    pub fn identity() -> Mat4 {
        Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // multiply this by other mat4 matrix
    pub fn multiply(&mut self, other: &Mat4) {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        self.m = result.m;
    }

    // translate the matrix by a vector
    pub fn translate(&mut self, Vector { x, y, z, w}: Vector) {
        self.m[0][3] += x;
        self.m[1][3] += y;
        self.m[2][3] += z;
    }

    // scales the matrix by a vector
    pub fn scale(&mut self, Vector { x, y, z, w}: Vector) {
        self.m[0][0] *= x;
        self.m[1][1] *= y;
        self.m[2][2] *= z;
    }

    // for information about this algorithm, see:
    // https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
    pub fn rotate(&mut self, angle: f64, Vector { x, y, z, w}: Vector) {
        let mut result = Mat4::new();
        let mut axis = Vector::new(x, y, z);

        axis.normalize();

        let sin = (angle as f64).sin();
        let cos = (angle as f64).cos();
        let t = 1.0 - cos;

        result.m[0][0] = t * axis.x * axis.x + cos;
        result.m[0][1] = t * axis.x * axis.y - sin * axis.z;
        result.m[0][2] = t * axis.x * axis.z + sin * axis.y;

        result.m[1][0] = t * axis.x * axis.y + sin * axis.z;
        result.m[1][1] = t * axis.y * axis.y + cos;
        result.m[1][2] = t * axis.y * axis.z - sin * axis.x;

        result.m[2][0] = t * axis.x * axis.z - sin * axis.y;
        result.m[2][1] = t * axis.y * axis.z + sin * axis.x;
        result.m[2][2] = t * axis.z * axis.z + cos;

        result.m[3][3] = 1.0;

        self.multiply(&result);
    }

    pub fn inverse(&mut self) -> bool {
        let mut out = Mat4::new();
        let mut m = self.m;

        let mut inv = [0.0; 16];
        let mut det = 0.0;

        inv[0] = m[1][1]  * m[2][2] * m[3][3] -
                 m[1][1]  * m[2][3] * m[3][2] -
                 m[2][1]  * m[1][2] * m[3][3] +
                 m[2][1]  * m[1][3] * m[3][2] +
                 m[3][1] * m[1][2] * m[2][3] -
                 m[3][1] * m[1][3] * m[2][2];

        inv[4] = -m[1][0]  * m[2][2] * m[3][3] +
                  m[1][0]  * m[2][3] * m[3][2] +
                  m[2][0]  * m[1][2] * m[3][3] -
                  m[2][0]  * m[1][3] * m[3][2] -
                  m[3][0] * m[1][2] * m[2][3] +
                  m[3][0] * m[1][3] * m[2][2];

        inv[8] = m[1][0] * m[2][1] * m[3][3] -
                m[1][0] * m[2][3] * m[3][1] -
                m[2][0] * m[1][1] * m[3][3] +
                m[2][0] * m[1][3] * m[3][1] +
                m[3][0] * m[1][1] * m[2][3] -
                m[3][0] * m[1][3] * m[2][1];

        inv[12] = -m[1][0] * m[2][1] * m[3][2] +
                   m[1][0] * m[2][2] * m[3][1] +
                   m[2][0] * m[1][1] * m[3][2] -
                   m[2][0] * m[1][2] * m[3][1] -
                   m[3][0] * m[1][1] * m[2][2] +
                   m[3][0] * m[1][2] * m[2][1];

        inv[1] = -m[0][1]  * m[2][2] * m[3][3] +
                    m[0][1]  * m[2][3] * m[3][2] +
                    m[2][1]  * m[0][2] * m[3][3] -
                    m[2][1]  * m[0][3] * m[3][2] -
                    m[3][1] * m[0][2] * m[2][3] +
                    m[3][1] * m[0][3] * m[2][2];

        inv[5] = m[0][0]  * m[2][2] * m[3][3] -
                m[0][0]  * m[2][3] * m[3][2] -
                m[2][0]  * m[0][2] * m[3][3] +
                m[2][0]  * m[0][3] * m[3][2] +
                m[3][0] * m[0][2] * m[2][3] -
                m[3][0] * m[0][3] * m[2][2];

        inv[9] = -m[0][0] * m[2][1] * m[3][3] +
                  m[0][0] * m[2][3] * m[3][1] +
                  m[2][0] * m[0][1] * m[3][3] -
                  m[2][0] * m[0][3] * m[3][1] -
                  m[3][0] * m[0][1] * m[2][3] +
                  m[3][0] * m[0][3] * m[2][1];

        inv[13] = m[0][0] * m[2][1] * m[3][2] -
                m[0][0] * m[2][2] * m[3][1] -
                m[2][0] * m[0][1] * m[3][2] +
                m[2][0] * m[0][2] * m[3][1] +
                m[3][0] * m[0][1] * m[2][2] -
                m[3][0] * m[0][2] * m[2][1];

        inv[2] = m[0][1] * m[1][2] * m[3][3] -
                m[0][1] * m[1][3] * m[3][2] -
                m[1][1] * m[0][2] * m[3][3] +
                m[1][1] * m[0][3] * m[3][2] +
                m[3][1] * m[0][2] * m[1][3] -
                m[3][1] * m[0][3] * m[1][2];

        inv[6] = -m[0][0] * m[1][2] * m[3][3] +
                  m[0][0] * m[1][3] * m[3][2] +
                  m[1][0] * m[0][2] * m[3][3] -
                  m[1][0] * m[0][3] * m[3][2] -
                  m[3][0] * m[0][2] * m[1][3] +
                  m[3][0] * m[0][3] * m[1][2];

        inv[10] = m[0][0] * m[1][1] * m[3][3] -
                m[0][0] * m[1][3] * m[3][1] -
                m[1][0] * m[0][1] * m[3][3] +
                m[1][0] * m[0][3] * m[3][1] +
                m[3][0] * m[0][1] * m[1][3] -
                m[3][0] * m[0][3] * m[1][1];

        inv[14] = -m[0][0] * m[1][1] * m[3][2] +
                   m[0][0] * m[1][2] * m[3][1] +
                   m[1][0] * m[0][1] * m[3][2] -
                   m[1][0] * m[0][2] * m[3][1] -
                   m[3][0] * m[0][1] * m[1][2] +
                   m[3][0] * m[0][2] * m[1][1];

        inv[3] = -m[0][1] * m[1][2] * m[2][3] +
                     m[0][1] * m[1][3] * m[2][2] +
                     m[1][1] * m[0][2] * m[2][3] -
                     m[1][1] * m[0][3] * m[2][2] -
                     m[2][1] * m[0][2] * m[1][3] +
                     m[2][1] * m[0][3] * m[1][2];

        inv[7] = m[0][0] * m[1][2] * m[2][3] -
                m[0][0] * m[1][3] * m[2][2] -
                m[1][0] * m[0][2] * m[2][3] +
                m[1][0] * m[0][3] * m[2][2] +
                m[2][0] * m[0][2] * m[1][3] -
                m[2][0] * m[0][3] * m[1][2];

        inv[11] = -m[0][0] * m[1][1] * m[2][3] +
                   m[0][0] * m[1][3] * m[2][1] +
                   m[1][0] * m[0][1] * m[2][3] -
                   m[1][0] * m[0][3] * m[2][1] -
                   m[2][0] * m[0][1] * m[1][3] +
                   m[2][0] * m[0][3] * m[1][1];

        inv[15] = m[0][0] * m[1][1] * m[2][2] -
                m[0][0] * m[1][2] * m[2][1] -
                m[1][0] * m[0][1] * m[2][2] +
                m[1][0] * m[0][2] * m[2][1] +
                m[2][0] * m[0][1] * m[1][2] -
                m[2][0] * m[0][2] * m[1][1];

        det = m[0][0] * inv[0] + m[0][1] * inv[4] + m[0][2] * inv[8] + m[0][3] * inv[12];

        if det == 0f64 {
            return false
        }

        det = 1.0 / det;

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = inv[i * 4 + j] * det;
            }
        }
        self.m = m;
        true
    }

    pub fn transpose(&mut self) {
        let mut m = self.m;
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = self.m[j][i];
            }
        }
        self.m = m;
    }

    // simple to_string for debugging purposes
    pub fn to_string(&self) -> String {
        let mut out: String = String::new();
        for i in 0..4 {
            out.push_str("[");
            for j in 0..4 {

                if self.m[i][j] < 0f64 {
                    out.push_str(&format!(" {:.3} ", self.m[i][j]));
                } else {
                    out.push_str(&format!("  {:.3} ", self.m[i][j]));
                }
            }
            out.push_str("]\n");
        }
        out
    }

}

// multiply by scalar
impl ops::Mul<f64> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: f64) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] * other;
            }
        }
        result
    }
}

impl ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] + other.m[i][j];
            }
        }
        result
    }
}

impl ops::AddAssign for Mat4 {
    fn add_assign(&mut self, other: Mat4) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] += other.m[i][j];
            }
        }
    }
}

impl ops::Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] - other.m[i][j];
            }
        }
        result
    }
}

impl ops::SubAssign for Mat4 {
    fn sub_assign(&mut self, other: Mat4) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] -= other.m[i][j];
            }
        }
    }
}

// multiply by scalar
impl ops::MulAssign<f64> for Mat4 {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] *= other;
            }
        }
    }
}

// multiply by matrix
impl ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }
}

// multiply by matrix
impl ops::MulAssign for Mat4 {
    fn mul_assign(&mut self, other: Mat4) {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        self.m = result.m;
    }
}

// check equality with other matrix
impl PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                // using comparison with epsilon + units of least precision
                if !approx_eq!(f64, self.m[i][j], other.m[i][j], F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
                    return false;
                }
            }
        }
        true
    }
}


// ###########
// ## TESTS ##
// ###########

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat4::identity();
        let identity = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, identity);
    }

    #[test]
    fn test_multiply() {
        let mut m = Mat4::identity();

        m.scale(Vector::new(3.0, 2.0, 5.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));

        let mut m2 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };

        m.multiply(&m2);

        let result = Mat4 {
            m: [
                [16.0, 20.0, 24.0, 28.0],
                [36.0, 40.0, 44.0, 48.0],
                [84.0, 92.0, 100.0, 108.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };

        assert_eq!(m, result);
    }

    #[test]
    fn test_translate() {
        use crate::vector::*;
        let mut m = Mat4::identity();
        let mut vec = Vector::new(1.0, 1.0, 1.0);
        m.translate(Vector::new(1.0, 2.0, 3.0));
        let result = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let vec_result = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(m, result);
        vec = vec * m;
        assert_eq!(vec, vec_result);
    }

    #[test]
    fn test_scale() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(1.0, 2.0, 3.0));
        let result = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 3.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn test_rotate() {
        let mut m = Mat4::identity();
        m.rotate(as_radians(90.0), Vector::new(0.0, 0.0, 1.0));
        let result = Mat4 {
            m: [
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);

        m = Mat4::identity();

        m.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));
        let result = Mat4 {
            m: [
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn inverse_test() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(2.0, 2.0, 2.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));
        m.inverse();
        let result = Mat4 {
            m: [
                [0.5, 0.0, 0.0, -0.5],
                [0.0, 0.5, 0.0, -1.0],
                [0.0, 0.0, 0.5, -1.5],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn transpose_test() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(2.0, 2.0, 2.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));
        m.transpose();
        let result = Mat4 {
            m: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [1.0, 2.0, 3.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn add_test() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(2.0, 2.0, 2.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));

        let mut m2 = Mat4::identity();
        m2.scale(Vector::new(2.0, 2.0, 2.0));
        m2.translate(Vector::new(1.0, 2.0, 3.0));

        m += m2;
        let result = Mat4 {
            m: [
                [4.0, 0.0, 0.0, 2.0],
                [0.0, 4.0, 0.0, 4.0],
                [0.0, 0.0, 4.0, 6.0],
                [0.0, 0.0, 0.0, 2.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn sub_test() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(2.0, 2.0, 2.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));
        let mut m2 = Mat4::identity();
        m2.scale(Vector::new(2.0, 2.0, 2.0));
        m2.translate(Vector::new(1.0, 2.0, 3.0));
        m -= m2;
        let result = Mat4 {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
        assert_eq!(m, result);
    }
}