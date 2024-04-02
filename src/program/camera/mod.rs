#![allow(dead_code)]
#[allow(unused_imports)]
// Linear Algebra types for transforming and creating matrices.
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec2, Vec3, Vec4},
};

// For arrays, not another type of Vector ->
use std::vec::Vec;

const X_UNIT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const Y_UNIT: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const Z_UNIT: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const CAMERA_SPEED: f32 = 25.0;
const PI: f32 = std::f32::consts::PI;

#[derive(Debug)]
pub struct Camera {
    // Where the camera is in world-space
    position: Vec3,
    // Where the camera is looking in world-space. This is the inverse of the unit Z vector of its
    // view space
    direction: Vec3,
    // Unit Y of View Space
    camera_y: Vec3,
    // Unit X of View Space
    camera_x: Vec3,
    // Cached View Matrix
    view_matrix: Mat4,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    pub(crate) fn new() -> Self {
        Self::look_at(ORIGIN, Z_UNIT)
    }

    // Generates a *view* matrix as if the virtual camera were at position and directed at the
    // target point
    pub fn look_at(position: Vec3, target: Vec3) -> Self {
        let direction = (position - target).normalized();
        let (camera_x, camera_y) = Self::find_x_y_axes(&direction);
        let view_matrix = Self::rotation_matrix(camera_x, camera_y, direction).transposed()
            * Self::translation_matrix_from_vec(-position);
        Camera {
            position,
            direction,
            camera_x,
            camera_y,
            view_matrix,
            pitch: 0.0,
            yaw: 90.0,
        }
    }

    fn find_x_y_axes(v: &Vec3) -> (Vec3, Vec3) {
        let camera_right = Y_UNIT.cross(*v).normalized();
        let camera_up = v.cross(camera_right);
        (camera_right, camera_up)
    }

    fn direction_from_pitch_and_yaw(pitch: f32, yaw: f32) -> Vec3 {
        let x = yaw.cos() * pitch.cos();
        let y = pitch.sin();
        let z = yaw.sin() * pitch.cos();
        Vec3::new(x, y, z).normalized()
    }

    pub fn update(
        &mut self,
        delta: &Vec<CameraMove>,
        delta_t: f32,
        x_offset: f64,
        y_offset: f64,
    ) -> () {
        let camera_speed = CAMERA_SPEED * delta_t;
        let rot_speed = camera_speed * 0.1;

        for change in delta.iter() {
            match change {
                CameraMove::Forwards => self.position += -self.direction * camera_speed,
                CameraMove::Backwards => self.position -= -self.direction * camera_speed,
                CameraMove::Left => {
                    self.position -=
                        -self.direction.cross(self.camera_y).normalized() * camera_speed
                }
                CameraMove::Right => {
                    self.position +=
                        -self.direction.cross(self.camera_y).normalized() * camera_speed
                }
                CameraMove::LookLeft => self.yaw -= rot_speed,
                CameraMove::LookRight => self.yaw += rot_speed,
                CameraMove::LookUp => self.pitch += rot_speed,
                CameraMove::LookDown => self.pitch -= rot_speed,

                _ => (),
            };
        }
        self.yaw += x_offset as f32;
        self.pitch += y_offset as f32;
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
        self.direction = -Self::direction_from_pitch_and_yaw(self.pitch, self.yaw);
        let (camera_x, camera_y) = Self::find_x_y_axes(&self.direction);
        self.camera_x = camera_x;
        self.camera_y = camera_y;

        // self.translation_matrix = Self::translation_matrix_from_vec(self.position);
        // self.rotation_matrix = Self::calculate_rotation_matrix(self.rotation_y, self.rotation_x);
        // self.matrix = self.rotation_matrix * self.translation_matrix;
    }

    pub fn view_matrix(&self) -> Mat4 {
        let rotation_matrix = self.rotation_view_matrix();
        let translation_matrix = self.translation_view_matrix();
        rotation_matrix * translation_matrix
    }

    // Feed it your unit vectors
    fn rotation_matrix(x: Vec3, y: Vec3, z: Vec3) -> Mat4 {
        Mat4::new(
            Vec4::new(x.x, x.y, x.z, 0.0),
            Vec4::new(y.x, y.y, y.z, 0.0),
            Vec4::new(z.x, z.y, z.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    fn rotation_view_matrix(&self) -> Mat4 {
        // Take inverse of the camera rotation matrix for the view matrix
        // Transpose of a rotation matrix is its inverse
        Self::rotation_matrix(self.camera_x, self.camera_y, self.direction).transposed()
    }

    fn translation_view_matrix(&self) -> Mat4 {
        // Don't forget to invert it for the View Matrix ;]
        Self::translation_matrix_from_vec(-self.position)
    }
    fn translation_matrix_from_vec(pos: Vec3) -> Mat4 {
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
    LookUp,
    LookDown,
    LookLeft,
    LookRight,
}
