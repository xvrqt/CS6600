// Hosts conversions from the idiosyncratic format of various 3D Mesh file formats and their
// parsers into our internal representation.
use super::{DrawStyle, Mesh, MeshError, Unattached};

// Linear algebra types we use in our internal representation
use ultraviolet::vec::Vec3;

use std::path::Path;

// Convenience Error Type
type Result<T> = std::result::Result<T, MeshError>;

// Extracts the vertices, normals, and UV coordinates
// Provides an implementation of Attribute that sets these up on a VAO
// It will use the DrawElements strategy of rendering
#[inline(always)]
pub(crate) fn load_mesh<P>(path: P) -> Result<Mesh<Unattached>>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .extension()
        .ok_or(MeshError::UnknownFileType("???".to_string()))
        .map(|os| os.to_str().unwrap())
        .and_then(|ext| match ext {
            "obj" => {
                let file = std::fs::read_to_string(path.as_ref())?;
                // TODO: Error needed here
                let mut objects = wavefront_obj::obj::parse(file).unwrap().objects;
                // TODO: Support multiple objects per parse
                let obj = objects.swap_remove(0);
                Ok(obj.into())
            }
            _ => Err(MeshError::UnknownFileType(ext.to_string())),
        })
}

// Wavefront Object (.obj)
impl From<wavefront_obj::obj::Object> for Mesh<Unattached> {
    fn from(obj: wavefront_obj::obj::Object) -> Self {
        // Construct a temporary vector that stores the per vertex data tuples so we can identify
        // repeated vertices and de-dupe them
        let mut vtn_tuples = Vec::new();

        // Generating these from the object info
        let st_coordinates = Vec::new();
        let mut normals = Vec::new();
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // TODO: Support multiple geometries per .obj file
        let shapes = &obj.geometry[0].shapes;

        // For each shape, destructure it into individual vertex information for remapping into
        // Element arrays.
        for shape in shapes.iter() {
            // We only support triangles for now
            // TODO: Make this an error instead of a panic! (Or support them lol)
            let triangle_points = match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(a, b, c) => vec![a, b, c],
                _ => panic!(),
            };

            // For each point, construct a tuple of the vertex information and check for matches
            for point in triangle_points {
                if let (v_index, _, Some(n_index)) = point {
                    // Grab the indices of the vertex information
                    let ele = (v_index, n_index);

                    // If the tuple exists, add that index to the element position array
                    if let Some(index) = vtn_tuples.iter().position(|&x| x == ele) {
                        indices.push(index as u32);
                    } else {
                        // Add that tuple, and add its index to the element position array
                        let index = vtn_tuples.len();
                        indices.push(index as u32);
                        vtn_tuples.push(ele);
                    }
                }
            }
        }

        // Go through the list of tuples, a split it into separate Vectors with the actual values
        // instead of the indices, and convert the types into our linear algebra types
        for (v_index, n_index) in vtn_tuples.iter() {
            let vertex = obj.vertices[*v_index];
            let vertex = Vec3::new(vertex.x as f32, vertex.y as f32, vertex.z as f32);
            vertices.push(vertex);

            let normal = obj.normals[*n_index];
            let normal = Vec3::new(normal.x as f32, normal.y as f32, normal.z as f32);
            normals.push(normal);
        }

        // Go through the list of vertices and record the largest/smallest point of every basis
        // (max_x, least_x, max_y, least_y, max_z, least_z)
        let mut boundary_points = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for vertex in vertices.iter() {
            let Vec3 { x, y, z } = vertex;
            if *x > boundary_points.0 {
                boundary_points.0 = *x;
            }
            if *x < boundary_points.1 {
                boundary_points.1 = *x;
            }
            if *y > boundary_points.2 {
                boundary_points.2 = *y;
            }
            if *y < boundary_points.3 {
                boundary_points.3 = *y;
            }
            if *z > boundary_points.4 {
                boundary_points.4 = *z;
            }
            if *y < boundary_points.5 {
                boundary_points.5 = *z;
            }
        }
        // Convert the boundary points into the 8 vertices of a rectangular prism
        let boundaries = [
            Vec3::new(boundary_points.1, boundary_points.2, boundary_points.5),
            Vec3::new(boundary_points.1, boundary_points.2, boundary_points.4),
            Vec3::new(boundary_points.0, boundary_points.2, boundary_points.5),
            Vec3::new(boundary_points.0, boundary_points.2, boundary_points.4),
            Vec3::new(boundary_points.1, boundary_points.3, boundary_points.5),
            Vec3::new(boundary_points.1, boundary_points.3, boundary_points.4),
            Vec3::new(boundary_points.0, boundary_points.3, boundary_points.5),
            Vec3::new(boundary_points.0, boundary_points.3, boundary_points.4),
        ];

        let name = obj.name;
        let draw_style = DrawStyle::Triangles;

        Mesh {
            name,
            draw_style,
            data: Unattached {
                vertices,
                normals,
                st_coordinates,
                indices,
                boundaries,
            },
        }
    }
}
