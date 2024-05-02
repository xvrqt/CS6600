#![allow(dead_code)]
mod projection;
pub use projection::Projection;

mod arcballcamera;
pub use arcballcamera::ArcBallCamera;

// Linear Algebra types for transforming and creating matrices.
use ultraviolet::{mat::Mat4, vec::Vec3};

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
    // Returns a pre-multiplied View-Projection Matrix, which is None if it hasn't changed since
    // the last read
    fn view_projection_matrix(&mut self) -> Option<Mat4>;
    // Returns where the camera is in World-Space
    fn position(&self) -> Vec3;
    // Takes a queue of CameraEvents and performs the necessary operations to update the View and
    // Projection matrices.
    fn update(&mut self, events: &mut VecDeque<CameraEvent>) -> ();
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
