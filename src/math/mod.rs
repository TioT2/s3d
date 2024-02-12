/// ANIM-RS Project
/// `File` util/math/mod.rs
/// `Description` Math main declaration module
/// `Author` TioT2
/// `Last changed` 17.12.2023

pub mod vec;
pub mod mat;

pub use vec::{Vec2, Vec3, Vec4};
pub use mat::Mat4x4;

pub type Mat4x4f = Mat4x4<f32>;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;

pub type Vec2u = Vec2<u32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4u = Vec4<u32>;

/// Floating point mat4 functionality re-exporting
mod mat4x4_f32;
pub use mat4x4_f32::*;

// file mod.rs