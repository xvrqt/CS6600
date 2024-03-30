use obj;
use ultraviolet::vec::{Vec2, Vec3, Vec4};

use crate::vao::SetAttributePointer;
use crate::vao::VAOError;

// Convert the parser's Obj type to our own.
// Theirs uses an Array of Structs (AoS) [bad]
// Ours uses a Struct of Arrays (SoA) [good]
// In addition, ours uses ultraviolet types to make computations fast, and
// to take advantage of the auto setup of the VAO Attribute Pointer
// This converts positions to homogenouse Vec4's, leaves normals a Vec3 and
// UVs as Vec2 (until I understand why they need a third; maybe texture index ?)
#[derive(Debug)]
pub struct Obj {
    pub vertices: Vec<Vec4>,
    pub normals: Vec<Vec3>,
    pub uv: Vec<Vec2>,
    // This struct draws it self with "DrawElements" so we need an array with
    // element indices
    pub indices: Vec<u16>,
}

// Convert from the .obj parser's format into ours
impl From<obj::Obj<obj::TexturedVertex, u16>> for Obj {
    fn from(o: obj::Obj<obj::TexturedVertex, u16>) -> Self {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv = Vec::new();

        // Destructure their struct
        for vnuv in o.vertices.iter() {
            let vertex = Vec4::new(vnuv.position[0], vnuv.position[1], vnuv.position[2], 1.0);
            let normal = Vec3::new(vnuv.normal[0], vnuv.normal[1], vnuv.normal[2]);
            let tex_coord = Vec2::new(vnuv.texture[0], vnuv.texture[1]);
            vertices.push(vertex);
            normals.push(normal);
            uv.push(tex_coord);
        }

        Obj {
            vertices,
            normals,
            uv,
            indices: o.indices,
        }
    }
}

impl Obj {}
