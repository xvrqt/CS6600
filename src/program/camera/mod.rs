#![allow(dead_code)]
use crate::program::projection::Projection;
use ultraviolet::projection;
#[allow(unused_imports)]
// Linear Algebra types for transforming and creating matrices.
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec2, Vec3, Vec4},
};

// For our queue of CameraEvents
use std::collections::VecDeque;

const PI: f32 = std::f32::consts::PI;
const CAMERA_DEFAULT_SPEED: f32 = 25.0;
const X_UNIT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const Y_UNIT: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const Z_UNIT: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

// Specific unit vector
#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

// Trait for different types of cameras
pub trait Camera {
    // Returns a Matrix which will transform points in World-Space to where they are from the
    // camera's point of view.
    fn view_matrix(&self) -> Mat4;
    // Returns a Matrix which will transform points in View-Space into the Canonical View-Volume.
    fn projection_matrix(&self) -> Mat4;
    // Takes a queue of CameraEvents and performs the necessary operations to update the View and
    // Projection matrices.
    fn update(&mut self, events: &mut VecDeque<CameraEvent>) -> ();
}

#[derive(Debug, Clone, Copy)]
pub struct ArcBallCamera {
    // Where the camera is pointing at in World-Space
    target: Vec3,
    // How far away from the target the camera is
    radius: f32,
    // The rotation matrix, representing the current rotation, and our x,y,z unit axes.
    rotation: Mat3,
    // Cached View Matrix
    view_matrix: Mat4,
    // Projection Enum, contains the matrix
    projection: Projection,
}

impl Default for ArcBallCamera {
    fn default() -> Self {
        ArcBallCamera {
            target: ORIGIN,
            radius: 25.0,
            rotation: Mat3::identity(),
            view_matrix: Mat4::identity(),
            projection: Projection::default_perspective(),
        }
    }
}

impl Camera for ArcBallCamera {
    fn view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }

    fn update(&mut self, events: &mut VecDeque<CameraEvent>) -> () {
        while let Some(event) = events.pop_front() {
            match event {
                CameraEvent::Movement(direction) => self.update_position(&direction),
                CameraEvent::Rotation(direction) => self.update_rotation(&direction),
                // Update Projection
                CameraEvent::ZoomProjection(mag) => self.projection = self.projection.zoom(mag),
                CameraEvent::SwapProjection => self.projection = self.projection.swap(),
                CameraEvent::ProjectionAspectRatio(new_aspect_ratio) => {
                    self.projection = self.projection.aspect_ratio(new_aspect_ratio)
                }
            };
        }
        // Update the View Matrix
        self.compute_view_matrix();
    }
}

impl ArcBallCamera {
    // New camera, which by default hovers above the origin
    pub(crate) fn new() -> Self {
        Self::default()
    }

    // Returns the camera's unit vector of the corresponding axis, convenience
    fn axis(&self, unit_vector: Axis) -> Vec3 {
        match unit_vector {
            Axis::X => self.rotation.cols[0],
            Axis::Y => self.rotation.cols[1],
            Axis::Z => self.rotation.cols[2],
        }
    }

    // Updates the camera's position in world-space
    fn update_position(&mut self, movement: &Direction) {
        match *movement {
            Direction::Forwards(_) => {
                self.radius -= 1.0;
                if self.radius < 0.11 {
                    self.radius = 0.11;
                }
            }

            Direction::Backwards(_) => {
                self.radius += 1.0;
            }
            // Direction::Left(mag) => self.position -= self.camera_x * camera_speed * mag,
            // Direction::Right(mag) => self.position += self.camera_x * camera_speed * mag,
            Direction::Center => {
                self.target = ORIGIN;
                self.radius = 35.0;
                self.rotation = Mat3::identity();
                self.projection = Projection::default_perspective();
            }

            Direction::Vector(x, y, _) => {
                self.target -= x * self.axis(Axis::X) * 25.0;
                self.target -= y * self.axis(Axis::Y) * 25.0;
            }
            _ => (),
        }
    }

    // Updates the camera's rotation in world-space
    fn update_rotation(&mut self, rotation: &Direction) {
        match *rotation {
            Direction::Up => {
                let x_axis = self.axis(Axis::X);
                let smidge = Mat3::from_rotation_around(x_axis, 0.05);
                self.rotation = smidge * self.rotation;
            }
            Direction::Down => {
                let x_axis = self.axis(Axis::X);
                let smidge = Mat3::from_rotation_around(x_axis, -0.05);
                self.rotation = smidge * self.rotation;
            }
            Direction::Left(_) => {
                let y_axis = self.axis(Axis::Y);
                let smidge = Mat3::from_rotation_around(y_axis, 0.05);
                self.rotation = smidge * self.rotation;
            }
            Direction::Right(_) => {
                let y_axis = self.axis(Axis::Y);
                let smidge = Mat3::from_rotation_around(y_axis, -0.05);
                self.rotation = smidge * self.rotation;
            }
            Direction::Vector(x, y, _) => {
                let x_axis = self.axis(Axis::X);
                let smidge = Mat3::from_rotation_around(x_axis, y);
                self.rotation = smidge * self.rotation;
                let y_axis = self.axis(Axis::Y);
                let smidge = Mat3::from_rotation_around(y_axis, -x);
                self.rotation = smidge * self.rotation;
            }
            _ => (),
        }
    }

    // Recomputes the View Matrix based on the updated: target, rotation, and radius
    fn compute_view_matrix(&mut self) -> () {
        let trans = self.inverse_translation_matrix();
        let rot = self.inverse_rotation_matrix();
        self.view_matrix = rot * trans;
    }

    // Inverse of a rotation matrix is its transpose. This takes 3 unit vectors of the camera and
    // returns the inverse matrix that represents that rotation in world-space. This can be used to
    // transform other, non-camera objects in world space into view-sapce
    fn inverse_rotation_matrix(&self) -> Mat4 {
        let x = self.axis(Axis::X);
        let y = self.axis(Axis::Y);
        let z = self.axis(Axis::Z);
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
        let position = (self.axis(Axis::Z) * self.radius) + self.target;
        let pos_x = position.x;
        let pos_y = position.y;
        let pos_z = position.z;
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
    SwapProjection,
    ProjectionAspectRatio(f32),
    ZoomProjection(f32),
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
