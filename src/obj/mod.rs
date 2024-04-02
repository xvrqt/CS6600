use obj;
use ultraviolet::vec::{Vec2, Vec3, Vec4};

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

        let indices = o.indices.iter().map(|e| *e as u16).collect();
        Obj {
            vertices,
            normals,
            uv,
            indices,
        }
    }
}

impl From<wavefront_obj::obj::Object> for Obj {
    fn from(o: wavefront_obj::obj::Object) -> Self {
        // For look ups
        let old_normals = o.normals;
        let old_uv = o.tex_vertices;
        println!("old normals length: {}", old_normals.len());
        println!("old uv length: {}", old_uv.len());
        println!("old vertices length: {}", o.vertices.len());
        println!("num triangles length: {}", o.geometry[0].shapes.len());
        let num_verts = o.vertices.len();

        // Generating these from the object info
        let mut uv = Vec::new();
        uv.resize(num_verts, Vec2::new(0.0, 0.0));
        let mut normals = Vec::new();
        normals.resize(num_verts, Vec3::new(0.0, 0.0, 0.0));
        let mut indices = Vec::new();

        for shape in o.geometry[0].shapes.iter() {
            let triangle_points = match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(a, b, c) => vec![a, b, c],
                _ => panic!(),
            };

            for point in triangle_points {
                if let (v_index, Some(t_index), Some(n_index)) = point {
                    // Vertex positions stay the same, "are canonical"
                    indices.push(v_index as u16);

                    // Look up the values in the corresponding arrays and remap them
                    let normal = old_normals[n_index];
                    let normal = Vec3::new(normal.x as f32, normal.y as f32, normal.z as f32);
                    normals[v_index] = normal;

                    let t = old_uv[t_index];
                    let t = Vec2::new(t.u as f32, t.v as f32);
                    uv[v_index] = t;
                }
            }
        }

        // The indices of these don't change so just convert them all in to the correct type
        let vertices = o
            .vertices
            .iter()
            .map(|e| Vec4::new(e.x as f32, e.y as f32, e.z as f32, 1.0))
            .collect();
        Obj {
            vertices,
            normals,
            uv,
            indices,
        }
    }
}

impl Obj {}
