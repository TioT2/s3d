/// ANIM-RS Project
/// `File` util/math/mat.rs
/// `Description` Math matrix implementation module
/// `Author` TioT2
/// `Last changed` 17.12.2023

use std::ops::{Mul, Add, Sub, Div, Neg};

/// 4x4 matrix types
pub struct Mat4x4<T> {
    pub data: [[T; 4]; 4],
} // struct Mat4x4<T>

impl<T> From<[[T; 4]; 4]> for Mat4x4<T> {
    /// From 4x4 array construction function
    /// * `data` - 4x4 array
    /// * Returns matrix contained by this array values
    fn from(data: [[T; 4]; 4]) -> Self {
        Self { data }
    } // fn from
} // impl From<[[T; 4]; 4] for Mat4x4<T>


impl<T: Clone> From<&[T; 16]> for Mat4x4<T> {
    /// 4x4 Matrix from linear array construction function
    /// * `data` - linear array
    /// * Returns matrix contained by this array values
    fn from(data: &[T; 16]) -> Self {
        Self {
            data: [
                [data[ 0].clone(), data[ 1].clone(), data[ 2].clone(), data[ 3].clone()],
                [data[ 4].clone(), data[ 5].clone(), data[ 6].clone(), data[ 7].clone()],
                [data[ 8].clone(), data[ 9].clone(), data[10].clone(), data[11].clone()],
                [data[12].clone(), data[13].clone(), data[14].clone(), data[15].clone()],
            ]
        }
    } // fn from
} // impl<T: Clone> From<&[T; 16]> for Mat4x4<T>

/// Unsafe 4x4 matrix operations implementation
impl<T> Mat4x4<T> {
    /// To linear slice conversion funciton
    /// * Returns contents of this matrix represented in linear slice
    pub fn as_linear_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts::<T>(std::mem::transmute::<*const [T; 4], *const T>(self.data.as_ptr()), 16)
        }
    } // fn as_linear_slice

    /// To mutable linear slice conversion funciton
    /// * Returns contents of this matrix represented in mutable linear slice
    pub fn as_mut_linear_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut::<T>(std::mem::transmute::<*mut [T; 4], *mut T>(self.data.as_mut_ptr()), 16)
        }
    } // fn as_mut_linear_slice
} // impl<T> Mat4x4<T>

/// Clone trait implementation
impl<T: Clone> Clone for Mat4x4<T> {
    /// Cloning function
    /// * Returns new matrix with exactly same contents
    fn clone(&self) -> Self {
        Self::from(self.data.clone())
    }
} // impl<T: Clone> Clone for Mat4x4<T>

/// Copy trait implementation
impl<T: Copy> Copy for Mat4x4<T> {

} // impl<T: Copy> Copy for Mat4x4<T>

/// Default trait implementation
impl<T: Default> Default for Mat4x4<T> {
    /// Default matrix create function
    /// * Returns matrix filled by default type values
    fn default() -> Self {
        Self {
            data: [
                [T::default(), T::default(), T::default(), T::default()],
                [T::default(), T::default(), T::default(), T::default()],
                [T::default(), T::default(), T::default(), T::default()],
                [T::default(), T::default(), T::default(), T::default()]
            ]
        }
    } // fn default
} // impl<T: Default> Default for Mat4x4<T>

/// Matrix multiplication operator implementation
impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> std::ops::Mul<Mat4x4<T>> for Mat4x4<T> {
    /// Output type - matrix
    type Output = Mat4x4<T>;

    /// Multiplication function itself
    /// * `rhs` - matrix to multiply this matrix on
    /// * Returns this matrix multiplied by rhs one
    fn mul(self, rhs: Mat4x4<T>) -> Self::Output {
        Self {
            data: [
                [
                    self.data[0][0] * rhs.data[0][0] + self.data[0][1] * rhs.data[1][0] + self.data[0][2] * rhs.data[2][0] + self.data[0][3] * rhs.data[3][0],
                    self.data[0][0] * rhs.data[0][1] + self.data[0][1] * rhs.data[1][1] + self.data[0][2] * rhs.data[2][1] + self.data[0][3] * rhs.data[3][1],
                    self.data[0][0] * rhs.data[0][2] + self.data[0][1] * rhs.data[1][2] + self.data[0][2] * rhs.data[2][2] + self.data[0][3] * rhs.data[3][2],
                    self.data[0][0] * rhs.data[0][3] + self.data[0][1] * rhs.data[1][3] + self.data[0][2] * rhs.data[2][3] + self.data[0][3] * rhs.data[3][3],
                ],
                [
                    self.data[1][0] * rhs.data[0][0] + self.data[1][1] * rhs.data[1][0] + self.data[1][2] * rhs.data[2][0] + self.data[1][3] * rhs.data[3][0],
                    self.data[1][0] * rhs.data[0][1] + self.data[1][1] * rhs.data[1][1] + self.data[1][2] * rhs.data[2][1] + self.data[1][3] * rhs.data[3][1],
                    self.data[1][0] * rhs.data[0][2] + self.data[1][1] * rhs.data[1][2] + self.data[1][2] * rhs.data[2][2] + self.data[1][3] * rhs.data[3][2],
                    self.data[1][0] * rhs.data[0][3] + self.data[1][1] * rhs.data[1][3] + self.data[1][2] * rhs.data[2][3] + self.data[1][3] * rhs.data[3][3],
                ],
                [
                    self.data[2][0] * rhs.data[0][0] + self.data[2][1] * rhs.data[1][0] + self.data[2][2] * rhs.data[2][0] + self.data[2][3] * rhs.data[3][0],
                    self.data[2][0] * rhs.data[0][1] + self.data[2][1] * rhs.data[1][1] + self.data[2][2] * rhs.data[2][1] + self.data[2][3] * rhs.data[3][1],
                    self.data[2][0] * rhs.data[0][2] + self.data[2][1] * rhs.data[1][2] + self.data[2][2] * rhs.data[2][2] + self.data[2][3] * rhs.data[3][2],
                    self.data[2][0] * rhs.data[0][3] + self.data[2][1] * rhs.data[1][3] + self.data[2][2] * rhs.data[2][3] + self.data[2][3] * rhs.data[3][3],
                ],
                [
                    self.data[3][0] * rhs.data[0][0] + self.data[3][1] * rhs.data[1][0] + self.data[3][2] * rhs.data[2][0] + self.data[3][3] * rhs.data[3][0],
                    self.data[3][0] * rhs.data[0][1] + self.data[3][1] * rhs.data[1][1] + self.data[3][2] * rhs.data[2][1] + self.data[3][3] * rhs.data[3][1],
                    self.data[3][0] * rhs.data[0][2] + self.data[3][1] * rhs.data[1][2] + self.data[3][2] * rhs.data[2][2] + self.data[3][3] * rhs.data[3][2],
                    self.data[3][0] * rhs.data[0][3] + self.data[3][1] * rhs.data[1][3] + self.data[3][2] * rhs.data[2][3] + self.data[3][3] * rhs.data[3][3],
                ],
            ],
        }
    } // fn mul
} // impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> std::ops::Mul<Mat4x4<T>> for Mat4x4<T>

/// Pack of operators definition module
impl<T: Copy + Neg<Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>> Mat4x4<T> {
    /// Determinant getting function
    /// * Returns determinant of this matrix
    pub fn determinant(&self) -> T {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] * self.data[3][3] + self.data[1][2] * self.data[2][3] * self.data[3][1] + self.data[1][3] * self.data[2][1] * self.data[3][2] - self.data[1][1] * self.data[2][3] * self.data[3][2] - self.data[1][2] * self.data[2][1] * self.data[3][3] - self.data[1][3] * self.data[2][2] * self.data[3][1]) -
        self.data[0][1] * (self.data[0][1] * self.data[2][2] * self.data[3][3] + self.data[0][2] * self.data[2][3] * self.data[3][1] + self.data[0][3] * self.data[2][1] * self.data[3][2] - self.data[0][1] * self.data[2][3] * self.data[3][2] - self.data[0][2] * self.data[2][1] * self.data[3][3] - self.data[0][3] * self.data[2][2] * self.data[3][1]) +
        self.data[0][2] * (self.data[0][1] * self.data[1][2] * self.data[3][3] + self.data[0][2] * self.data[1][3] * self.data[3][1] + self.data[0][3] * self.data[1][1] * self.data[3][2] - self.data[0][1] * self.data[1][3] * self.data[3][2] - self.data[0][2] * self.data[1][1] * self.data[3][3] - self.data[0][3] * self.data[1][2] * self.data[3][1]) -
        self.data[0][3] * (self.data[0][1] * self.data[1][2] * self.data[2][3] + self.data[0][2] * self.data[1][3] * self.data[2][1] + self.data[0][3] * self.data[1][1] * self.data[2][2] - self.data[0][1] * self.data[1][3] * self.data[2][2] - self.data[0][2] * self.data[1][1] * self.data[2][3] - self.data[0][3] * self.data[1][2] * self.data[2][1])
    } // fn determinant

    /// Matrix inversion getting function
    /// * Returns this matrix inersed
    pub fn inversed(&self) -> Self {
        let determ_00 = self.data[1][1] * self.data[2][2] * self.data[3][3] + self.data[1][2] * self.data[2][3] * self.data[3][1] + self.data[1][3] * self.data[2][1] * self.data[3][2] - self.data[1][1] * self.data[2][3] * self.data[3][2] - self.data[1][2] * self.data[2][1] * self.data[3][3] - self.data[1][3] * self.data[2][2] * self.data[3][1];
        let determ_01 = self.data[0][1] * self.data[2][2] * self.data[3][3] + self.data[0][2] * self.data[2][3] * self.data[3][1] + self.data[0][3] * self.data[2][1] * self.data[3][2] - self.data[0][1] * self.data[2][3] * self.data[3][2] - self.data[0][2] * self.data[2][1] * self.data[3][3] - self.data[0][3] * self.data[2][2] * self.data[3][1];
        let determ_02 = self.data[0][1] * self.data[1][2] * self.data[3][3] + self.data[0][2] * self.data[1][3] * self.data[3][1] + self.data[0][3] * self.data[1][1] * self.data[3][2] - self.data[0][1] * self.data[1][3] * self.data[3][2] - self.data[0][2] * self.data[1][1] * self.data[3][3] - self.data[0][3] * self.data[1][2] * self.data[3][1];
        let determ_03 = self.data[0][1] * self.data[1][2] * self.data[2][3] + self.data[0][2] * self.data[1][3] * self.data[2][1] + self.data[0][3] * self.data[1][1] * self.data[2][2] - self.data[0][1] * self.data[1][3] * self.data[2][2] - self.data[0][2] * self.data[1][1] * self.data[2][3] - self.data[0][3] * self.data[1][2] * self.data[2][1];

        let determ =
            self.data[0][0] * determ_00 -
            self.data[0][1] * determ_01 +
            self.data[0][2] * determ_02 -
            self.data[0][3] * determ_03;

        Self {
            data: [
                [
                     self.data[0][0] * determ_00 / determ,
                    -self.data[0][1] * determ_01 / determ,
                     self.data[0][2] * determ_02 / determ,
                    -self.data[0][3] * determ_03 / determ,
                ],
                [
                    -self.data[1][0] * (self.data[0][1] * self.data[2][2] * self.data[3][3] + self.data[0][2] * self.data[2][3] * self.data[3][1] + self.data[0][3] * self.data[2][1] * self.data[3][2] - self.data[0][1] * self.data[2][3] * self.data[3][2] - self.data[0][2] * self.data[2][1] * self.data[3][3] - self.data[0][3] * self.data[2][2] * self.data[3][1]) / determ,
                     self.data[1][1] * (self.data[0][0] * self.data[2][2] * self.data[3][3] + self.data[0][2] * self.data[2][3] * self.data[3][0] + self.data[0][3] * self.data[2][0] * self.data[3][2] - self.data[0][0] * self.data[2][3] * self.data[3][2] - self.data[0][2] * self.data[2][0] * self.data[3][3] - self.data[0][3] * self.data[2][2] * self.data[3][0]) / determ,
                    -self.data[1][2] * (self.data[0][0] * self.data[2][1] * self.data[3][3] + self.data[0][1] * self.data[2][3] * self.data[3][0] + self.data[0][3] * self.data[2][0] * self.data[3][1] - self.data[0][0] * self.data[2][3] * self.data[3][1] - self.data[0][1] * self.data[2][0] * self.data[3][3] - self.data[0][3] * self.data[2][1] * self.data[3][0]) / determ,
                     self.data[1][3] * (self.data[0][0] * self.data[2][1] * self.data[3][2] + self.data[0][1] * self.data[2][2] * self.data[3][0] + self.data[0][2] * self.data[2][0] * self.data[3][1] - self.data[0][0] * self.data[2][2] * self.data[3][1] - self.data[0][1] * self.data[2][0] * self.data[3][2] - self.data[0][2] * self.data[2][1] * self.data[3][0]) / determ,
                ],
                [
                     self.data[2][0] * (self.data[0][1] * self.data[1][2] * self.data[3][3] + self.data[0][2] * self.data[1][3] * self.data[3][1] + self.data[0][3] * self.data[1][1] * self.data[3][2] - self.data[0][1] * self.data[1][3] * self.data[3][2] - self.data[0][2] * self.data[1][1] * self.data[3][3] - self.data[0][3] * self.data[1][2] * self.data[3][1]) / determ,
                    -self.data[2][1] * (self.data[0][0] * self.data[1][2] * self.data[3][3] + self.data[0][2] * self.data[1][3] * self.data[3][0] + self.data[0][3] * self.data[1][0] * self.data[3][2] - self.data[0][0] * self.data[1][3] * self.data[3][2] - self.data[0][2] * self.data[1][0] * self.data[3][3] - self.data[0][3] * self.data[1][2] * self.data[3][0]) / determ,
                     self.data[2][2] * (self.data[0][0] * self.data[1][1] * self.data[3][3] + self.data[0][1] * self.data[1][3] * self.data[3][0] + self.data[0][3] * self.data[1][0] * self.data[3][1] - self.data[0][0] * self.data[1][3] * self.data[3][1] - self.data[0][1] * self.data[1][0] * self.data[3][3] - self.data[0][3] * self.data[1][1] * self.data[3][0]) / determ,
                    -self.data[2][3] * (self.data[0][0] * self.data[1][1] * self.data[3][2] + self.data[0][1] * self.data[1][2] * self.data[3][0] + self.data[0][2] * self.data[1][0] * self.data[3][1] - self.data[0][0] * self.data[1][2] * self.data[3][1] - self.data[0][1] * self.data[1][0] * self.data[3][2] - self.data[0][2] * self.data[1][1] * self.data[3][0]) / determ,
                ],
                [
                    -self.data[3][0] * (self.data[0][1] * self.data[1][2] * self.data[2][3] + self.data[0][2] * self.data[1][3] * self.data[2][1] + self.data[0][3] * self.data[1][1] * self.data[2][2] - self.data[0][1] * self.data[1][3] * self.data[2][2] - self.data[0][2] * self.data[1][1] * self.data[2][3] - self.data[0][3] * self.data[1][2] * self.data[2][1]) / determ,
                     self.data[3][1] * (self.data[0][0] * self.data[1][2] * self.data[2][3] + self.data[0][2] * self.data[1][3] * self.data[2][0] + self.data[0][3] * self.data[1][0] * self.data[2][2] - self.data[0][0] * self.data[1][3] * self.data[2][2] - self.data[0][2] * self.data[1][0] * self.data[2][3] - self.data[0][3] * self.data[1][2] * self.data[2][0]) / determ,
                    -self.data[3][2] * (self.data[0][0] * self.data[1][1] * self.data[2][3] + self.data[0][1] * self.data[1][3] * self.data[2][0] + self.data[0][3] * self.data[1][0] * self.data[2][1] - self.data[0][0] * self.data[1][3] * self.data[2][1] - self.data[0][1] * self.data[1][0] * self.data[2][3] - self.data[0][3] * self.data[1][1] * self.data[2][0]) / determ,
                     self.data[3][3] * (self.data[0][0] * self.data[1][1] * self.data[2][2] + self.data[0][1] * self.data[1][2] * self.data[2][0] + self.data[0][2] * self.data[1][0] * self.data[2][1] - self.data[0][0] * self.data[1][2] * self.data[2][1] - self.data[0][1] * self.data[1][0] * self.data[2][2] - self.data[0][2] * self.data[1][1] * self.data[2][0]) / determ,
                ],
           ]
        }
    } // fn inversed
} // impl<T: Copy + Neg<Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>> Mat4x4<T>

// file mat.rs