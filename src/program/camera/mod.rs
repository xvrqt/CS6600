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
    // Unit Z of View Space (inverse of 'direction')
    camera_z: Vec3,
    // Unit X of View Space
    camera_x: Vec3,
    // Unit Y of View Space
    camera_y: Vec3,
    // Cached View Matrix
    view_matrix: Mat4,
}

impl Camera {
    pub(crate) fn new() -> Self {
        let position = Vec3::new(0.0, 0.0, 5.0);
        let target = ORIGIN;
        Self::look_at(position, target)
    }

    // Returns the "front" of the camera, or the direction the camera is pointing
    fn direction(&self) -> Vec3 {
        -self.camera_z
    }

    // Generates a *view* matrix as if the virtual camera were at position and directed at the
    // target point in world-space
    pub fn look_at(position: Vec3, target: Vec3) -> Self {
        let camera_z = -(target - position).normalized();
        let (camera_x, camera_y) = Self::find_x_y_axes(&camera_z);
        println!("x: {:#?}\ny: {:#?}\nz: {:#?}", camera_x, camera_x, camera_z);
        Camera {
            position,
            camera_x,
            camera_y,
            camera_z,
            view_matrix: Mat4::look_at(position, target, Y_UNIT),
        }
    }

    fn find_x_y_axes(v: &Vec3) -> (Vec3, Vec3) {
        let camera_right = v.cross(Y_UNIT).normalized();
        let camera_up = v.cross(camera_right);
        (camera_right, camera_up)
    }

    pub fn update(&mut self, delta_t: f32, camera_events: &Vec<CameraMove>) -> () {
        let camera_speed = CAMERA_SPEED * delta_t;
        let rot_speed = camera_speed * 0.1;

        for change in camera_events.iter() {
            match change {
                CameraMove::Forwards => self.position -= self.camera_z * camera_speed,
                CameraMove::Backwards => self.position += self.camera_z * camera_speed,
                CameraMove::Left => {
                    self.position -= self.camera_z.cross(self.camera_y).normalized() * camera_speed
                }
                CameraMove::Right => {
                    self.position += self.camera_z.cross(self.camera_y).normalized() * camera_speed
                }
                // CameraMove::LookLeft => self.yaw -= rot_speed,
                // CameraMove::LookRight => self.yaw += rot_speed,
                // CameraMove::LookUp => self.pitch += rot_speed,
                // CameraMove::LookDown => self.pitch -= rot_speed,
                //
                _ => (),
            };
        }
        let (camera_x, camera_y) = Self::find_x_y_axes(&self.camera_z);
        self.camera_x = camera_x;
        self.camera_y = camera_y;
    }

    // Return the view matrix that should be used to transform the world-space coordinates into
    // view-space
    pub fn view_matrix(&self) -> Mat4 {
        let rotation_matrix = self.inverse_rotation_matrix();
        let translation_matrix = self.inverse_translation_matrix();
        rotation_matrix * translation_matrix
    }

    // Inverse of a rotation matrix is its transpose. This takes 3 unit vectors of the camera and
    // returns the inverse matrix that represents that rotation in world-space. This can be used to
    // transform other, non-camera objects in world space into view-sapce
    fn inverse_rotation_matrix(&self) -> Mat4 {
        let x = self.camera_x;
        let y = self.camera_y;
        let z = self.camera_z;
        Mat4::new(
            Vec4::new(x.x, x.y, x.z, 0.0),
            Vec4::new(y.x, y.y, y.z, 0.0),
            Vec4::new(z.x, z.y, z.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
        .transposed()
    }

    fn inverse_translation_matrix(&self) -> Mat4 {
        /* Inverse Translation Matrix
         * +-        -+
         * | 1 0 0 -x |
         * | 0 1 0 -y |
         * | 0 0 1 -z |
         * | 0 0 0  1 |
         * +-        -+
         */
        let pos_x = self.position.x;
        let pos_y = self.position.y;
        let pos_z = self.position.z;
        Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(-pos_x, -pos_y, -pos_z, 1.0),
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

// fn direction_from_pitch_and_yaw(pitch: f32, yaw: f32) -> Vec3 {
//     let x = yaw.cos() * pitch.cos();
//     let y = pitch.sin();
//     let z = yaw.sin() * pitch.cos();
//     Vec3::new(x, y, z).normalized()
// }
// self.yaw += x_offset as f32;
// self.pitch += y_offset as f32;
// if self.pitch > 89.0 {
//     self.pitch = 89.0;
// }
// if self.pitch < -89.0 {
//     self.pitch = -89.0;
// }
// self.direction = -Self::direction_from_pitch_and_yaw(self.pitch, self.yaw);
