// Converts various 3D file types into our internal represntation 'Mesh'
mod conversions;
use super::GLDraw;
use super::{scene_object::SceneObject, vao::VAO};
pub use crate::program::Attribute;

// Linear algebra types we use in our internal representation
use gl::types::*;
use ultraviolet::vec::{Vec2, Vec3};

// Error Types
mod error;
pub use error::MeshError;
type Result<T> = std::result::Result<T, MeshError>;

// Standard Library
use std::path::Path;
use std::rc::Weak;

// Internal format for 3D Models
#[derive(Debug, Clone)]
pub struct Mesh<State> {
    pub(crate) name: String,
    pub(crate) data: State,
    pub(crate) draw_style: DrawStyle,
}

// Internal representation of a 3D model.
#[derive(Debug, Clone)]
pub struct Unattached {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) st_coordinates: Vec<Vec2>,
    pub(crate) indices: Vec<u32>,
    pub(crate) boundaries: [Vec3; 8],
}

// We buffer the mesh data to the GPU and drop it to free memory, leaving only an OpenGL VAO in its
// place.
#[derive(Debug, Clone)]
pub struct Attached {
    pub(crate) vao: VAO,
    pub(crate) objects: Vec<Weak<SceneObject>>,
    pub(crate) boundaries: [Vec3; 8],
    pub(crate) program_id: GLuint,
}

// Used for setting the OpenGL Draw Style. Wraps types in the `gl` crate so the library caller
// doesn't have to use the `gl` crate.
#[derive(Debug, Clone)]
pub enum DrawStyle {
    Triangles,
    Points,
}

impl DrawStyle {
    fn value(&self) -> GLuint {
        match *self {
            DrawStyle::Triangles => gl::TRIANGLES,
            DrawStyle::Points => gl::POINTS,
        }
    }
}

// Uses OpenGL `DrawElementsInstanced()` and so checks each SceneObject to see if it's enabled, and
// if so, adds its `transform` matrix to a varying to buffered alongside the mesh vertice and
// normal varyings.
impl GLDraw for Mesh<Attached> {
    fn draw(&mut self) -> super::Result<()> {
        // Iterate over all the scene objects, checking if they're enabled, and collating their
        // transforms into a contiguous array to be buffered to the GPU
        let mut transforms = Vec::new();
        let mut normal_transforms = Vec::new();
        for object in self.data.objects.iter() {
            // The Scene Object may have been removed
            // TODO: Handle the removal by changing the Vec of SceneObjects
            if let Some(object) = object.upgrade() {
                if object.enabled {
                    // Only update the transform if it has changed
                    // TODO: Only update the transform data if it has changed / partially buffer it
                    if let Some(transform) = object.transform {
                        transforms.push(transform);
                    }

                    if let Some(normals) = object.normal_transform {
                        normal_transforms.push(normals);
                    }
                }
            }
        }
        // Buffer the transform data to the GPU
        if transforms.len() > 1 {
            self.data
                .vao
                .update_attribute("object_mw_transforms", &transforms, true)?;
        }
        if normal_transforms.len() > 1 {
            self.data.vao.update_attribute(
                "object_mw_normal_transforms",
                &normal_transforms,
                true,
            )?;
        }

        let vao = &self.data.vao;
        // This might be wrong at some point - i.e. if we start doing partial buffer updates
        let num_instances = transforms.len() as i32;
        unsafe {
            gl::BindVertexArray(vao.id);
            // TODO: Update VAO struct to elements_buffer.id (idk, that implies it's more than an
            // id i guess, which it's not :s)
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vao.elements.buffer_id);
            gl::DrawElementsInstanced(
                self.draw_style.value(),
                vao.elements.buffer_length,
                gl::UNSIGNED_INT, // Must match the size of the elements_buffer!
                std::ptr::null(),
                num_instances,
            );
        }
        Ok(())
    }
}

// Meshes can be created outside of a GLProgram (to save overhead in reading from disk and parsing)
// and attached to multiple GLPrograms. GLPrograms must `attach()` meshes so that they generate a
// Vertex Attrib Object that can be used to render them. They cannot do this without knowing which
// GLProgram they are attaching to.
impl Mesh<Unattached> {
    // Called by a GLProgram to attach this mesh. Takes the mesh data and buffers it to the GPU,
    // and then drops it. Replaces it with an OpenGL VAO that knows how to draw itself.
    pub(crate) fn attach(self, program_id: GLuint) -> Result<Mesh<Attached>> {
        // Move everything except data - which we unpack to buffer to the GPU
        let Mesh {
            name,
            data:
                Unattached {
                    vertices,
                    normals,
                    st_coordinates,
                    indices,
                    boundaries,
                },
            draw_style,
            ..
        } = self;

        // Create a new OpenGL VAO from our vertex data
        let mut vao = VAO::new(program_id, &indices)?;
        // Attach these buffers as attributes when creating our VAO
        let object_transforms = vec![ultraviolet::Mat4::identity()];
        let object_normals = vec![ultraviolet::Mat3::identity()];
        vao.add_attribute("vertices", &vertices, false)?;
        vao.add_attribute("normals", &normals, false)?;
        vao.add_attribute("object_mw_transforms", &object_transforms, false)?;
        vao.add_attribute("object_mw_normal_transforms", &object_normals, false)?;

        let objects = Vec::new();
        let data = Attached {
            vao,
            objects,
            boundaries,
            program_id,
        };

        Ok(Mesh {
            name,
            data,
            draw_style,
        })
    }

    // Updates the draw style for OpenGL drawArrays/Elements/Instances calls
    pub fn set_draw_style(&mut self, draw_style: DrawStyle) -> () {
        self.draw_style = draw_style;
    }

    // Load a mesh from a Path (wrapper so we don't need to `use` all the various parsers in this
    // module too)
    pub fn parse<P>(path: P) -> Result<Mesh<Unattached>>
    where
        P: AsRef<Path>,
    {
        conversions::load_mesh(path)
    }
}
