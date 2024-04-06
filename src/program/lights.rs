use ultraviolet::vec::{Vec3, Vec4};

#[derive(Debug, Copy, Clone)]
pub struct LightColor {
    r: f32,
    g: f32,
    b: f32,
    intensity: f32,
}

impl LightColor {
    pub fn new(r: f32, g: f32, b: f32, intensity: f32) -> Self {
        let r = r.clamp(0.0, 1.0);
        let g = g.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);
        let intensity = intensity.clamp(0.0, 1.0);
        LightColor { r, g, b, intensity }
    }

    pub(crate) fn to_vec4(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.intensity)
    }

    pub const WHITE: Self = LightColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        intensity: 1.0,
    };
    pub const RED: Self = LightColor {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        intensity: 1.0,
    };
    pub const GREEN: Self = LightColor {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        intensity: 1.0,
    };
    pub const BLUE: Self = LightColor {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        intensity: 1.0,
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position { x, y, z }
    }

    pub(crate) fn to_vec4(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
    }

    pub(crate) fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub const ORIGIN: Self = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightSource {
    color: Vec4,
    position: Vec4,
}

impl LightSource {
    pub fn new(color: &LightColor, position: &Position) -> Self {
        LightSource {
            color: Vec4::new(color.r, color.g, color.b, color.intensity),
            position: Vec4::new(position.x, position.y, position.z, 1.0),
        }
    }
}
