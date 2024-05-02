// Camera Trait, and common enums
use super::{Axis, Camera, CameraEvent, Direction, Projection, ORIGIN};

// Linear Algebra types for storing our vectors and transforms
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec3, Vec4},
};

// For the CameraEvents
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct ArcBallCamera {
    // Where the camera is pointing at in World-Space
    target: Vec3,
    // How far away from the target the camera is
    radius: f32,
    // The rotation matrix, representing the current rotation, and our x,y,z unit axes.
    rotation: Mat3,
    // The camera's position in World-Space
    position: Vec3,
    // Scene View Matrix (world transform into this camera's POV)
    view_matrix: Mat4,
    // Projection Enum, contains the Projection Matrix
    projection: Projection,
    // Cached View-Projection Matrix; caller sets to "None" when they take it so they know it
    // hasn't changed and can skip sending it to the GPU again
    pub(crate) view_projection_matrix: Option<Mat4>,
}

impl Default for ArcBallCamera {
    fn default() -> Self {
        let projection = Projection::default_perspective();
        let projection_matrix = projection.matrix();
        ArcBallCamera {
            target: ORIGIN,
            radius: 25.0,
            rotation: Mat3::identity(),
            position: Vec3::new(0.0, 0.0, 0.0),
            view_matrix: Mat4::identity(),
            projection,
            view_projection_matrix: Some(projection_matrix),
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

    fn view_projection_matrix(&mut self) -> Option<Mat4> {
        self.view_projection_matrix.take()
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn update(&mut self, events: &mut VecDeque<CameraEvent>) -> () {
        let update_required = events.len() > 0;
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
        if update_required {
            self.compute_view_matrix();
        }
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
            Direction::Forwards(mag) => {
                self.radius -= 1.0 * mag;
                if self.radius < 0.11 {
                    self.radius = 0.11;
                }
            }

            Direction::Backwards(mag) => {
                self.radius += 1.0 * mag;
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
        // Calculate the camera World-Space transform
        let rotation = self.rotation_matrix();
        let translation = self.translation_matrix();
        let camera_transform = translation * rotation;
        // Update the camera's position while we're at it
        self.position = camera_transform.cols[3].truncated();

        // Inverse is the view matrix
        self.view_matrix = camera_transform.inversed();

        // Update the cache
        let m = self.projection.matrix() * self.view_matrix;
        self.view_projection_matrix = Some(m);
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

    fn rotation_matrix(&self) -> Mat4 {
        let x = self.axis(Axis::X);
        let y = self.axis(Axis::Y);
        let z = self.axis(Axis::Z);
        Mat4::new(
            Vec4::new(x.x, x.y, x.z, 0.0),
            Vec4::new(y.x, y.y, y.z, 0.0),
            Vec4::new(z.x, z.y, z.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    // Inverse of the matrix that translates the camera into it's position in world-space
    fn inverse_translation_matrix(&self) -> Mat4 {
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

    fn translation_matrix(&self) -> Mat4 {
        let position = (self.axis(Axis::Z) * self.radius) + self.target;
        let pos_x = position.x;
        let pos_y = position.y;
        let pos_z = position.z;
        Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(pos_x, pos_y, pos_z, 1.0),
        )
    }
}
