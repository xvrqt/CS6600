#![allow(dead_code)]
#[allow(unused_imports)]
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec2, Vec3, Vec4},
};

use std::vec::Vec;

#[derive(Debug)]
pub struct Camera {
    rotation_y: f32,
    rotation_x: f32,
    rotation_matrix: Mat4,
    direction: Vec4,
    position: Vec4,
    translation_matrix: Mat4,
    pub matrix: Mat4,
}

impl Camera {
    pub(crate) fn new() -> Self {
        Camera {
            rotation_y: 0.0,
            rotation_x: 0.0,
            rotation_matrix: Mat4::identity(),
            direction: Vec4::new(0.0, 0.0, 1.0, 0.0),
            position: Vec4::new(0.0, 0.0, 0.0, 1.0),
            translation_matrix: Mat4::identity(),
            matrix: Mat4::identity(),
        }
    }

    pub(crate) fn with(position: Vec3, direction: Vec3) -> Self {
        Camera {
            rotation_y: 0.0,
            rotation_x: 0.0,
            rotation_matrix: Mat4::identity(),
            direction: Vec4::new(direction.x, direction.y, direction.z, 0.0),
            position: Vec4::new(position.x, position.y, position.z, 1.0),
            translation_matrix: Mat4::identity(),
            matrix: Mat4::identity(),
        }
    }

    pub fn update(&mut self, delta: &Vec<CameraMove>) -> () {
        for change in delta.iter() {
            match change {
                CameraMove::Forwards => self.position += self.direction,
                CameraMove::Backwards => self.position -= self.direction,
                _ => (),
            };
        }
        self.translation_matrix = Self::translation_matrix_from_vec(self.position);
        self.matrix = self.translation_matrix * self.rotation_matrix;
    }

    fn calculate_rotation_matrix(rot_y: f32, rot_x: f32) -> Mat4 {
        let rot_ym = Mat4::from_rotation_y(rot_y);
        let rot_xm = Mat4::from_rotation_x(rot_x);
        rot_ym * rot_xm
    }

    fn translation_matrix_from_vec(pos: Vec4) -> Mat4 {
        Self::calculate_translation_matrix(pos.x, pos.y, pos.z)
    }
    fn calculate_translation_matrix(pos_x: f32, pos_y: f32, pos_z: f32) -> Mat4 {
        /* Translation Matrix
         * +-       -+
         * | 1 0 0 0 |
         * | 0 1 0 0 |
         * | 0 0 1 0 |
         * | 0 0 0 1 |
         * +-       -+
         */
        Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(pos_x, pos_y, pos_z, 1.0),
        )
    }
}

pub enum CameraMove {
    Up,
    Down,
    Left,
    Right,
    Forwards,
    Backwards,
}
