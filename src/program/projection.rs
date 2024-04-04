use ultraviolet::mat::Mat4;

const PI: f32 = std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Projection {
    Ortho {
        side: f32,
        // Width / Height
        aspect_ratio: f32,
        matrix: Mat4,
    },
    Perspective {
        // Vertical FOV
        fov: f32,
        // Width / Height
        aspect_ratio: f32,
        // Should be positive, Ultraviolet expects z inverted
        z_near: f32,
        z_far: f32,
        matrix: Mat4,
    },
}

impl Projection {
    // Default Orthographic Projection
    pub(crate) fn default_ortho() -> Projection {
        Self::new_ortho(10.0, 1.0)
    }

    // New Orthographic Projection
    pub(crate) fn new_ortho(side: f32, aspect_ratio: f32) -> Projection {
        Projection::Ortho {
            side,
            aspect_ratio,
            matrix: ultraviolet::projection::orthographic_gl(-side, side, -side, side, 0.1, side),
        }
    }

    // Default Perspective Projection
    pub(crate) fn default_perspective() -> Projection {
        let fov = PI / 3.0;
        Self::new_perspective(fov, 1.0, 0.1, 10000.0)
    }

    // New Perspective Projection
    pub(crate) fn new_perspective(
        fov: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Projection {
        Projection::Perspective {
            fov: PI / 3.0,
            aspect_ratio: 1.0,
            z_near: 0.1,
            z_far: 10000.0,
            matrix: ultraviolet::projection::perspective_gl(fov, aspect_ratio, z_near, z_far),
        }
    }

    // Converts the Projection to Orthographic
    pub(crate) fn ortho(self) -> Self {
        match self {
            Projection::Ortho { .. } => self,
            Projection::Perspective {
                fov,
                aspect_ratio,
                z_near,
                z_far,
                matrix,
            } => {
                let side = z_near * (fov / 2.0).tan();
                Self::new_ortho(side, aspect_ratio)
            }
        }
    }

    // Converts the Projection to Perspective
    pub(crate) fn perspective(self) -> Self {
        match self {
            Projection::Perspective { .. } => self,
            Projection::Ortho {
                side,
                aspect_ratio,
                matrix,
            } => {
                let fov = (side / 0.1).atan() * 2.0;
                Self::new_perspective(fov, aspect_ratio, 0.1, side)
            }
        }
    }

    // Returns the other type of projection
    pub(crate) fn swap(self) -> Self {
        match self {
            Projection::Ortho { .. } => self.perspective(),
            Projection::Perspective { .. } => self.ortho(),
        }
    }

    // Returns the projection matrix
    pub(crate) fn matrix(&self) -> Mat4 {
        match self {
            Projection::Ortho { matrix, .. } => *matrix,
            Projection::Perspective { matrix, .. } => *matrix,
        }
    }
}
