use std::rc::Rc;
use ultraviolet::vec::{Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Material {
    specular_coeficient: f32,
    color_ambient: Vec3,
    color_diffuse: Vec3,
    color_specular: Vec3,
}

impl Material {
    pub fn new(
        color_ambient: Vec3,
        color_diffuse: Vec3,
        color_specular: Vec3,
        specular_coeficient: f32,
    ) -> Self {
        Self {
            specular_coeficient,
            color_ambient,
            color_diffuse,
            color_specular,
        }
    }
}
