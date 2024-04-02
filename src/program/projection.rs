use ultraviolet::mat::Mat4;

const PI: f32 = std::f32::consts::PI;

#[derive(Debug)]
pub enum Projection {
    Ortho {
        side: f32,
        aspect_ratio: f32,
    },
    Perspective {
        fov: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    },
}

impl Projection {
    // Return the matric to correctly project our scene based of projection type and associated
    // data
    pub(crate) fn mat(&self) -> Mat4 {
        match *self {
            Projection::Ortho {
                side: s,
                aspect_ratio: ar,
            } => ultraviolet::projection::orthographic_gl(-s * ar, s * ar, -s, s, 0.1, s),
            Projection::Perspective {
                fov,
                aspect_ratio,
                z_near,
                z_far,
            } => ultraviolet::projection::perspective_gl(fov, aspect_ratio, z_near, z_far),
        }
    }

    pub(crate) fn default_ortho() -> Projection {
        Projection::Ortho {
            side: 10.0,
            aspect_ratio: 1.0,
        }
    }

    pub(crate) fn default_perspective() -> Projection {
        Projection::Perspective {
            fov: PI / 3.0,
            aspect_ratio: 1.0,
            // Hate how these are positive instead of negative :s what a terrible convetion tbh
            z_near: 0.1,
            z_far: 10000.0,
        }
    }
}
