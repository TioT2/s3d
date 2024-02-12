/// ANIM-RS Project
/// `File` util/math/mat.rs
/// `Description` Math 4x4 floating-point matrix implementation module
/// `Author` TioT2
/// `Last changed` 17.12.2023

use super::vec::*;
use super::mat::*;

/// Default matrices implementation
impl Mat4x4<f32> {
    /// Identity matrix getting function
    /// * Returns identity matrix
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    } // fn identity

    /// Rotation matrix getting function
    /// * `angle` - angle to create rotation matrix on
    /// * `axis` - axis to create rotation matrix based on
    /// * Returns rotation matrix
    pub fn rotate(angle: f32, mut axis: Vec3<f32>) -> Self {
        axis.normalize();

        let sina = angle.sin();
        let cosa = angle.cos();

        Self {
            data: [
                [axis.x * axis.x * (1.0 - cosa) + cosa,            axis.x * axis.y * (1.0 - cosa) - axis.z * sina,   axis.x * axis.z * (1.0 - cosa) + axis.y * sina,   0.0],
                [axis.y * axis.x * (1.0 - cosa) + axis.z * sina,   axis.y * axis.y * (1.0 - cosa) + cosa,            axis.y * axis.z * (1.0 - cosa) - axis.x * sina,   0.0],
                [axis.z * axis.x * (1.0 - cosa) - axis.y * sina,   axis.z * axis.y * (1.0 - cosa) + axis.x * sina,   axis.z * axis.z * (1.0 - cosa) + cosa,            0.0],
                [0.0,                                              0.0,                                              0.0,                                              1.0]
            ]
        }
    } // fn rotate

    pub fn rotate_x(angle: f32) -> Self
    {
        let sin = angle.sin();
        let cos = angle.cos();

        Self {
            data: [
                [1.0,  0.0, 0.0, 0.0],
                [0.0,  cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0,  0.0, 0.0, 1.0]
            ]
        }
    } // fn rotate_x

    pub fn rotate_y(angle: f32) -> Self
    {

        let sin = angle.sin();
        let cos = angle.cos();

        Self {
            data: [
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0,  0.0, 0.0],
                [sin, 0.0,  cos, 0.0],
                [0.0, 0.0,  0.0, 1.0],
            ]
        }
    } // fn rotate_y

    pub fn rotate_z(angle: f32) -> Self
    {
        let sin = angle.sin();
        let cos = angle.cos();

        Self {
            data: [
                [ cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ]
        }
    } // fn rotate_z

    pub fn scale(scx: f32, scy: f32, scz: f32) -> Self {
        Self {
            data: [
                [scx, 0.0, 0.0, 0.0],
                [0.0, scy, 0.0, 0.0],
                [0.0, 0.0, scz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    } // fn scale

    pub fn translate(tdx: f32, tdy: f32, tdz: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [tdx, tdy, tdz, 1.0],
            ]
        }
    } // fn translate
} // impl Mat4x4<f32>

/// Projection functions implementation
impl Mat4x4<f32> {
    pub fn projection_ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Mat4x4<f32> {
        Self {
            data: [
                [2.0 / (r - l),      0.0,                0.0,                0.0],
                [0.0,                2.0 / (t - b),      0.0,                0.0],
                [0.0,                0.0,                -2.0 / (f - n),     0.0],
                [-(r + l) / (r - l), -(t + b) / (t - b), -(f + n) / (f - n), 1.0]
            ]
        }
    } // fn projection_ortho

    pub fn projection_frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Mat4x4<f32> {
        Self {
            data: [
                [2.0 * n / (r - l),   0.0,                  0.0,                   0.0],
                [0.0,                -2.0 * n / (t - b),    0.0,                   0.0],
                [(r + l) / (r - l),   (t + b) / (t - b),   -(f + n) / (f - n),    -1.0],
                [0.0,                 0.0,                 -2.0 * n * f / (f - n), 0.0]
            ]
        }
    } // fn projection_frustum

    pub fn view(loc: &Vec3<f32>, at: &Vec3<f32>, approx_up: &Vec3<f32>) -> Mat4x4<f32> {
        let dir = (*at - *loc).normalized();
        let right = (dir % *approx_up).normalized();
        let up = (right % dir).normalized();

        Self {
            data: [
                [right.x,         up.x,         -dir.x,       0.0],
                [right.y,         up.y,         -dir.y,       0.0],
                [right.z,         up.z,         -dir.z,       0.0],
                [-loc.dot(right), -loc.dot(up), loc.dot(dir), 1.0],
            ]
        }
    } // fn view
} // impl Mat4x4<f32>

impl Mat4x4<f32> {
    pub fn transform_vector(&self, v: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: v.x * self.data[0][0] + v.y * self.data[1][0] + v.z * self.data[2][0],
            y: v.x * self.data[0][1] + v.y * self.data[1][1] + v.z * self.data[2][1],
            z: v.x * self.data[0][2] + v.y * self.data[1][2] + v.z * self.data[2][2],
        }
    } // fn transform_vector

    pub fn transform_point(&self, v: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: v.x * self.data[0][0] + v.y * self.data[1][0] + v.z * self.data[2][0] + self.data[3][0],
            y: v.x * self.data[0][1] + v.y * self.data[1][1] + v.z * self.data[2][1] + self.data[3][1],
            z: v.x * self.data[0][2] + v.y * self.data[1][2] + v.z * self.data[2][2] + self.data[3][2],
        }
    } // fn transform_point

    pub fn transform_4x4(&self, v: Vec3<f32>) -> Vec3<f32> {
        let w = v.x * self.data[0][3] + v.y * self.data[1][3] + v.z * self.data[2][3] + self.data[3][3];

        Vec3 {
            x: (v.x * self.data[0][0] + v.y * self.data[1][0] + v.z * self.data[2][0] + self.data[3][0]) / w,
            y: (v.x * self.data[0][1] + v.y * self.data[1][1] + v.z * self.data[2][1] + self.data[3][1]) / w,
            z: (v.x * self.data[0][2] + v.y * self.data[1][2] + v.z * self.data[2][2] + self.data[3][2]) / w,
        }
    } // En transform_4x4
}

// file mat4x4_f32.rs