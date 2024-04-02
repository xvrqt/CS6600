// #![allow(dead_code)]
// Import and Re-Export our Error Type
pub mod error;
use ::glfw::Context;
pub use error::ProgramError;
pub mod builder;
pub mod projection;
pub use projection::Projection;
pub use window::FrameState;

use crate::window::GLWindow;
type Result<T> = std::result::Result<T, ProgramError>;

pub mod camera;
use crate::window;

// Programs must attach at least a Vertex and Fragment Shader
use crate::shader::{
    blinn_phong, FragmentShader, GeometryShader, Shader, ShaderPipeline, TesselationShader,
    VertexShader,
};
// Convenient use of special types that work well with OpenGL
use crate::types::*;
// Create and set uniform shader values
use crate::uniform::{MagicUniform, Uniform};
// Create and manager OpenGL Vertex Attribute Objects
use crate::vao::VAO;
// Special per frame values used in the draw() call

use crate::obj::Obj;

// OpenGL Types
use gl::types::*;

// Used to track Vertex Array Objects
use std::collections::hash_map::Entry;
use std::collections::HashMap;
// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
use std::ffi::CString;
use ultraviolet::mat::{Mat3, Mat4};
use ultraviolet::vec::{Vec3, Vec4};

use self::camera::Camera;

const SIDE: f32 = 1.0;
const ORIGINV3: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[repr(C)]
#[derive(Debug)]
struct LightSource {
    color: Vec4,
    position: Vec4,
}

// Semantic OpenGL Program
#[derive(Debug)]
#[allow(dead_code)]

// V, F -> Shader Type (built-in shaders have special implementations to make things easier)
pub struct GLProgram<'a, Type> {
    // OpenGL Program ID
    id: u32,
    // Window, Events, and OpenGL context
    context: GLWindow,
    // OpenGL Shaders, e.g. vertex, fragment, et al.
    shaders: ShaderPipeline<'a>,
    camera: Camera,
    projection: Projection,
    lights: Vec<LightSource>,
    lights_buffer: Option<GLuint>,
    // List of uniforms we update automagically for the caller
    // magic_uniforms: MagicUniform,
    // // List of VAOs to render
    vaos: HashMap<String, VAO>,
    // // List of light sources
    _pd: std::marker::PhantomData<Type>,
}

// Dummy types that represent different GLPrograms with different abilities built-in
#[derive(Debug)]
pub struct CustomShader;
#[derive(Debug)]
pub struct BlinnPhong;

impl<'a> GLProgram<'a, BlinnPhong> {}

// Functions commong to all GLProgram types
impl<'a, Any> GLProgram<'a, Any> {
    // Update the Programs Projection Matrix
    pub fn use_projection(&mut self, projection: Projection) -> () {
        self.projection = projection;
    }

    // Sets a uniform variable at the location
    pub fn set_uniform<S, Type>(&self, name: S, mut value: Type) -> Result<()>
    where
        S: AsRef<str>,
        Type: Uniform,
    {
        unsafe {
            gl::UseProgram(self.id);
        }
        let location = self.get_uniform_location(name)?;
        value
            .set(location)
            .map_err(|e| ProgramError::SettingUniformValue(e.to_string()))?;

        Ok(())
    }

    // Convenience function to look up uniform locatoin
    fn get_uniform_location<S>(&self, name: S) -> Result<GLint>
    where
        S: AsRef<str>,
    {
        let c_name = CString::new(name.as_ref()).map_err(|_| {
            ProgramError::SettingUniformValue(
                "Could not create CString from the uniform's location name.".to_string(),
            )
        })?;
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.id, c_name.into_raw());
        }
        match location {
            -1 => Err(ProgramError::GetUniformLocation(name.as_ref().into())),
            _ => Ok(location),
        }
    }

    // Similar to get_uniform_location but for block indices
    fn get_uniform_block_index<S>(&self, name: S) -> Result<GLuint>
    where
        S: AsRef<str>,
    {
        let c_name = CString::new(name.as_ref()).map_err(|_| {
            ProgramError::SettingUniformValue(
                "Could not create CString from the uniform's location name.".to_string(),
            )
        })?;
        let location;
        unsafe {
            location = gl::GetUniformBlockIndex(self.id, c_name.into_raw());
        }
        match location {
            gl::INVALID_INDEX => Err(ProgramError::GetUniformLocation(name.as_ref().into())),
            _ => Ok(location),
        }
    }
}

// Used to create a GLProgram
impl<'a> GLProgram<'a, BlinnPhong> {
    // Adds a light to the scene
    pub fn add_light(&mut self, position: Vec3, color: Vec3) -> Result<()> {
        // Transform into homogeneous 3D coordinates, and add full opacity to the color value
        let light = LightSource {
            color: Vec4::new(color.x, color.y, color.z, 1.0),
            position: Vec4::new(position.x, position.y, position.z, 1.0),
        };
        self.lights.push(light);
        // Since we're buffering to the GPU we don't want extra mememory at the end
        self.lights.shrink_to_fit();

        // Get location of the lights on the GPU, and length of the array CPU side
        let block_id = self.get_uniform_block_index("Lights")?;
        let num_lights = self.lights.len();

        // Rebind the lights (This is the layount in the shader code)
        let binding_point = 1 as GLuint;

        // Generate a Uniform Buffer if we haven't alreaady
        let buffer_id = match self.lights_buffer {
            Some(id) => id,
            None => {
                let mut id = 0 as GLuint;
                unsafe {
                    gl::GenBuffers(1, &mut id);
                }
                self.lights_buffer = Some(id);
                id
            }
        };
        // Buffer the data
        unsafe {
            gl::UniformBlockBinding(self.id, block_id, binding_point);
            let ptr = self.lights.as_ptr() as *const std::ffi::c_void;
            // Could be error site
            let size = (num_lights * std::mem::size_of::<LightSource>()) as GLsizeiptr;
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id);
            gl::BufferData(gl::UNIFORM_BUFFER, size, ptr, gl::DYNAMIC_DRAW);
        }
        // Bind the buffer and the block to the same place
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, buffer_id);
        }
        self.set_uniform("num_lights", GL1U(num_lights as GLuint))?;
        Ok(())
    }

    // Set the ambient light for the scene
    pub fn ambient_light(&mut self, color: Vec3, intensity: GLfloat) -> Result<()> {
        let color = GL4F(color.x, color.y, color.z, 1.0);
        let intensity = intensity.clamp(0.0, 1.0);
        let intensity = GL1F(intensity);
        self.set_uniform("ambient_light_color", color)?;
        self.set_uniform("ambient_intensity", intensity)
    }

    // Create a new, or edit an existing, VAO
    pub fn vao<S>(&mut self, name: S) -> &mut VAO
    where
        S: AsRef<str>,
    {
        // Return existing, or create a new VAO
        match self.vaos.entry(name.as_ref().to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(VAO::new(self.id)),
        }
    }

    // Create a new, or edit an existing, VAO
    pub fn vao_from_obj<S>(&mut self, name: S, obj: &Obj) -> Result<&mut Self>
    where
        S: AsRef<str>,
    {
        // Return existing, or create a new VAO
        let vao = match self.vaos.entry(name.as_ref().to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(VAO::new(self.id)),
        };
        // Set up the VAO state to use indices
        let mut ele_buffer = 0;
        let ele_buffer_ptr = obj.indices.as_ptr() as *const std::ffi::c_void;
        let ele_buffer_size = (obj.indices.len() * std::mem::size_of::<u16>()) as isize;
        unsafe {
            gl::GenBuffers(1, &mut ele_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ele_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ele_buffer_size,
                ele_buffer_ptr,
                gl::STATIC_DRAW,
            );
        }
        vao.ele_buffer = Some(ele_buffer);
        vao.attribute("vertices", obj.vertices.clone())?;
        vao.attribute("normals", obj.normals.clone())?;
        // vao.attribute("texture_coords", obj.uv)?;
        vao.draw_count = obj.indices.len() as GLint;
        Ok(self)
    }

    // Draws the next frame of the program
    pub fn draw(&mut self) -> Result<()> {
        // Set OpenGL State for this Program
        unsafe {
            gl::UseProgram(self.id);
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::BlendFunc(gl::ONE, gl::ONE);
            gl::Enable(gl::DEPTH_TEST);
            // gl::DepthFunc(gl::ALWAYS);
            // gl::CullFace(gl::BACK);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let frame_events = self.context.process_events();

        // Update the projection if the aspect ratio has changed
        // Toggle projection if P was pressed

        // Set uniforms for Vertex perspective transform, and vertex and normal Model-View
        // Transform for inside the Blinn-Phong Vertex Shader
        let (mvp, mv, mvn) = self.generate_view_matrices();
        self.set_uniform("mvp", mvp)?;
        self.set_uniform("mvn", mvn)?;
        self.set_uniform("mv", mv)?;

        for vao in self.vaos.values() {
            if vao.enabled {
                unsafe {
                    gl::BindVertexArray(vao.id);
                    if let Some(ele_buffer) = vao.ele_buffer {
                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ele_buffer);
                        gl::DrawElements(
                            vao.draw_style,
                            vao.draw_count,
                            gl::UNSIGNED_SHORT,
                            std::ptr::null(),
                        );
                    } else {
                        gl::DrawArrays(vao.draw_style, 0, vao.draw_count);
                    }
                }
            }
        }
        self.context.window.swap_buffers();
        self.context.glfw.poll_events();
        Ok(())
    }

    // Help function to generate the view matrices used for setting vertex position and normals for
    // Blinn-Phong Shaders
    pub(crate) fn generate_view_matrices(&self) -> (Mat4, Mat4, Mat3) {
        // Set uniforms for camera position
        let mvp = self.projection.mat() * self.camera.view_matrix();
        let mv = self.camera.view_matrix();
        let mut mvn: ultraviolet::mat::Mat3 = mv.truncate();
        mvn.inverse();
        mvn.transpose();

        (mvp, mv, mvn)
    }
}

//
// // Enables a magic uniform value
// pub fn enable_uniform(mut self, uniform: MagicUniform) -> Self {
//     self.magic_uniforms = self.magic_uniforms | uniform;
//     self
// }
//
// // Checks which magic uniforms are enabled and then sets them accordingly
// fn update_magic_uniforms(&self, vars: &FrameState) -> Result<()> {
//     if self.magic_uniforms.contains(MagicUniform::TIME) {
//         self.set_uniform("time", GL1F(vars.time))?;
//     }
//     if self.magic_uniforms.contains(MagicUniform::RESOLUTION) {
//         if let Some((x, y)) = vars.resolution {
//             self.set_uniform("resolution", GL2F(x, y))?;
//         }
//     }
//     Ok(())
// }
