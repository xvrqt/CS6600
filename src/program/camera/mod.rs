#![allow(dead_code)]
use crate::program::projection::Projection;
use ultraviolet::projection;
#[allow(unused_imports)]
// Linear Algebra types for transforming and creating matrices.
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec2, Vec3, Vec4},
};

// For arrays, not another type of Vector ->
use std::vec::Vec;

const PI: f32 = std::f32::consts::PI;
const CAMERA_DEFAULT_SPEED: f32 = 25.0;
const X_UNIT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const Y_UNIT: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const Z_UNIT: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Debug)]
pub struct Camera {
    // Where the camera is in world-space
    position: Vec3,
    target: Vec3,
    radius: f32,
    // Scales how fast the camera moves
    speed: f32,
    // Unit X of View Space
    camera_x: Vec3,
    // Unit Y of View Space
    camera_y: Vec3,
    // Unit Z of View Space (inverse of 'direction')
    camera_z: Vec3,
    // Cached View Matrix
    view_matrix: Option<Mat4>,
    // Projection
    projection: Projection,
    rot_x: f32,
    rot_y: f32,
    rot: Mat3,
}

impl Camera {
    // New camera, which by default hovers above the origin
    pub(crate) fn new() -> Self {
        let position = Vec3::new(5.0, 5.0, 5.0);
        let target = ORIGIN;
        let mut c = Camera {
            position,
            rot: Mat3::from_euler_angles(0.0, 0.0, 0.0),
            target,
            radius: 15.0,
            speed: CAMERA_DEFAULT_SPEED,
            camera_x: X_UNIT,
            camera_y: Y_UNIT,
            camera_z: Z_UNIT,
            view_matrix: None,
            projection: Projection::default_perspective(),
            rot_x: 0.0,
            rot_y: 0.0,
        };
        c.look_at(position, target);
        c
    }

    // Returns the "front" of the camera, or the direction the camera is pointing
    fn direction(&self) -> Vec3 {
        -self.camera_z
    }

    // Generates a new Camera as if the virtual camera were at position and directed at the
    // target point in world-space
    pub fn look_at(&mut self, position: Vec3, target: Vec3) -> () {
        // Update Rotation
        self.camera_z = -(target - position).normalized();
        self.update_axes();
        // Update Position
        self.update_position(&Direction::Absolute(position));
    }

    // Updates the camera's position in world-space
    fn update_position(&mut self, movement: &Direction) {
        let camera_speed = self.speed;
        match *movement {
            Direction::Forwards(_) => {
                self.position -= self.camera_z;
                self.radius -= 1.0;
                if self.radius < 0.0 {
                    self.radius = 0.0;
                }
            }
            Direction::Backwards(_) => {
                self.position += self.camera_z;
                self.radius += 1.0;
            }
            // Direction::Left(mag) => self.position -= self.camera_x * camera_speed * mag,
            // Direction::Right(mag) => self.position += self.camera_x * camera_speed * mag,
            Direction::Absolute(pos) => self.position = pos,
            Direction::Center => {
                self.radius = 35.0;
                self.rot_y = 0.0;
                self.rot_x = 0.0;
                self.position = Vec3::new(0.0, 0.0, 15.0);
                self.target = ORIGIN;
                self.camera_z = Z_UNIT;
            }
            Direction::Vector(x, y, _) => {
                self.position -= x * self.camera_x * camera_speed;
                self.target -= x * self.camera_x * camera_speed;
                self.position -= y * self.camera_y * camera_speed;
                self.target -= y * self.camera_y * camera_speed;
            }
            Direction::Flip => {
                self.camera_x = -self.camera_x;
            }
            _ => (),
        }
        self.update_axes();
        self.view_matrix = None;
    }

    // Updates the camera's rotation in world-space
    fn update_rotation(&mut self, rotation: &Direction) {
        match *rotation {
            Direction::Up => {
                let x_axis = self.rot.cols[0];
                let smidge = Mat3::from_rotation_around(x_axis, 0.05);
                self.rot = smidge * self.rot;

                println!("rot2: {:#?}", self.rot);
                // let rotor = ultraviolet::rotor::Rotor3::from_rotation_yz(0.05).normalized();
                // rotor.rotate_vec(&mut self.position);
            }
            Direction::Down => {
                let x_axis = self.rot.cols[0];
                let smidge = Mat3::from_rotation_around(x_axis, -0.05);
                self.rot = smidge * self.rot;
                // let rotor = ultraviolet::rotor::Rotor3::from_rotation_yz(-0.05).normalized();
                // rotor.rotate_vec(&mut self.position);
            }
            Direction::Left(_) => {
                let y_axis = self.rot.cols[1];
                let smidge = Mat3::from_rotation_around(y_axis, 0.05);
                self.rot = smidge * self.rot;
                // let rotor = ultraviolet::rotor::Rotor3::from_rotation_xz(-0.05).normalized();
                // rotor.rotate_vec(&mut self.position);
            }
            Direction::Right(_) => {
                let y_axis = self.rot.cols[1];
                let smidge = Mat3::from_rotation_around(y_axis, -0.05);
                self.rot = smidge * self.rot;
                // let rotor = ultraviolet::rotor::Rotor3::from_rotation_xy(0.05).normalized();
                // rotor.rotate_vec(&mut self.position);
            }
            // Direction::Vector(x, y, _) => {
            //     let tug = tug_factor * ((self.camera_x * x) + (self.camera_y * y));
            //     println!("tug: {:#?}", tug);
            //     self.camera_z = (self.camera_z - tug).normalized();
            //     self.update_axes();
            // }
            _ => (),
        }
        self.update_axes();
        self.view_matrix = None;
    }

    fn update_projection(&mut self, zoom: &Direction) -> () {
        self.projection = match *zoom {
            Direction::In(mag) | Direction::Out(mag) => match self.projection {
                Projection::Perspective {
                    fov,
                    aspect_ratio,
                    z_near,
                    z_far,
                } => {
                    let mut fov = fov + (mag * 0.1);
                    if fov > PI {
                        fov = PI
                    }
                    if fov < 0.1 {
                        fov = 0.1
                    }
                    Projection::Perspective {
                        fov,
                        aspect_ratio,
                        z_near,
                        z_far,
                    }
                }
                Projection::Ortho { side, aspect_ratio } => {
                    let mut side = side - mag;
                    if side > 100.0 {
                        side = 100.0
                    }
                    if side < 1.0 {
                        side = 1.0
                    }
                    Projection::Ortho { side, aspect_ratio }
                }
            },
            Direction::Ratio(aspect_ratio) => match self.projection {
                Projection::Perspective {
                    fov, z_near, z_far, ..
                } => Projection::Perspective {
                    fov,
                    aspect_ratio,
                    z_near,
                    z_far,
                },
                Projection::Ortho { side, .. } => Projection::Ortho { side, aspect_ratio },
            },
            _ => Projection::default_perspective(),
        };
    }

    // Updates the unit axez based on the camera's Z unit vector
    fn update_axes(&mut self) -> () {
        self.camera_z = -(self.target - self.position).normalized();
        self.camera_y = self.camera_z.cross(X_UNIT).normalized();
        self.camera_x = self.camera_z.cross(self.camera_y).normalized();
        self.view_matrix = None;
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.mat()
    }

    pub fn update(&mut self, delta_t: f32, camera_events: &Vec<CameraEvent>) -> () {
        for change in camera_events.iter() {
            match change {
                CameraEvent::Movement(direction) => self.update_position(direction),
                CameraEvent::Rotation(direction) => self.update_rotation(direction),
                CameraEvent::Projection(zoom) => self.update_projection(zoom),
            };
        }
        // Only update orthogonal axes, and invalidate the view matrix if it changed
        if camera_events.len() > 0 {
            self.update_axes();
        }
    }

    // Return the view matrix that should be used to transform the world-space coordinates into
    // view-space
    pub fn view_matrix(&mut self) -> Mat4 {
        if let Some(view_matrix) = self.view_matrix {
            view_matrix
        } else {
            let view_matrix = self.compute_view_matrix();
            self.view_matrix = Some(view_matrix);
            view_matrix
        }
    }

    fn compute_view_matrix(&mut self) -> Mat4 {
        // let rotation_matrix = self.inverse_rotation_matrix();
        // let translation_matrix = self.inverse_translation_matrix();
        // rotation_matrix * translation_matrix
        // let rot = self.rot;
        self.camera_x = self.rot.cols[0];
        self.camera_y = self.rot.cols[1];
        self.camera_z = self.rot.cols[2];
        self.position = (self.rot.cols[2] * self.radius) + self.target;
        // println!("self: {:#?}", self);
        // self.position = self.target + position;

        // self.update_axes();
        // ultraviolet::mat::Mat4::look_at(self.positioc, ORIGIN, self.camera_y)
        let trans = self.inverse_translation_matrix();
        let rot = self.inverse_rotation_matrix();
        rot * trans
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

    // Inverse of the matrix that translates the camera into it's position in world-space
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

#[derive(Debug)]
pub enum CameraEvent {
    Movement(Direction),
    Rotation(Direction),
    Projection(Direction),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    In(f32),
    Out(f32),
    Left(f32),
    Right(f32),
    Forwards(f32),
    Backwards(f32),
    Absolute(Vec3),
    Vector(f32, f32, f32),
    Ratio(f32),
    LookUp,
    LookDown,
    LookLeft,
    LookRight,
    Center,
    Flip,
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
