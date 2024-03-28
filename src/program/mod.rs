#![allow(unused_imports)]
#![allow(dead_code)]
// Import and Re-Export our Error Type
pub mod error;
pub use error::ProgramError;

// // Allows the caller to build their own GLProgram
// pub mod builder;
// use crate::program::builder::{GLProgramBuilder, NoFS, NoVS};

// Programs must attach at least a Vertex and Fragment Shader
use crate::shader::{
    BlinnPhongFragmentShader, BlinnPhongVertexShader, CustomFragmentShader, CustomVertexShader,
    Fragment, Shader, Vertex,
};
// Convenient use of special types that work well with OpenGL
use crate::types::*;
// Create and set uniform shader values
use crate::uniform::{MagicUniform, Uniform};
// Create and manager OpenGL Vertex Attribute Objects
use crate::vao::VAO;
// Special per frame values used in the draw() call
use crate::FrameState;

use crate::obj::Obj;

// OpenGL Types
use gl::types::*;

// Used to track Vertex Array Objects
use std::collections::hash_map::Entry;
use std::collections::HashMap;
// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
use std::ffi::CString;

// Structs used to type a GLProgram and restrict some implementations

// Semantic OpenGL Program
#[derive(Debug)]
#[allow(dead_code)]
// V, F -> Shader Type (built-in shaders have special implementations to make things easier)
pub struct GLProgram<V, F> {
    id: u32, // OpenGL keeps track of programs with integer IDs
    // We never read these again, but I can imagine a future where we would want to
    vertex_shader: V,
    fragment_shader: F,
    // List of uniforms we update automagically for the caller
    magic_uniforms: MagicUniform,
    // List of VAOs to render
    vaos: HashMap<String, VAO>,
}

// Used to create a GLProgram
impl<'a> GLProgram<Shader<'_, Vertex>, Shader<'_, Fragment>> {
    // Create a new OpenGL Program using custom shaders
    pub fn new(
        vertex_shader: CustomVertexShader<'a>,
        fragment_shader: CustomFragmentShader<'a>,
    ) -> Result<GLProgram<CustomVertexShader<'a>, CustomFragmentShader<'a>>, ProgramError> {
        let id;
        unsafe {
            id = gl::CreateProgram();

            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);

            gl::LinkProgram(id);
        }

        // If it was a success, return the Blinn-Phong builder
        link_shaders_success(id).and_then(|_| {
            Ok(GLProgram {
                id,
                vertex_shader,
                fragment_shader,
                magic_uniforms: MagicUniform::NONE,
                vaos: HashMap::new(),
            })
        })
    }

    // Creates a new OpenGL Program using a built-in Blinn-Phong shader
    // Return type provides additional functions to more easily manage scenes
    pub fn blinn_phong_shading(
    ) -> Result<GLProgram<BlinnPhongVertexShader<'a>, BlinnPhongFragmentShader<'a>>, ProgramError>
    {
        // Compile Blinn-Phong Shaders
        let vertex_shader = Shader::<Vertex>::blinn_phong()?;
        let fragment_shader = Shader::<Fragment>::blinn_phong()?;

        GLProgram::new(vertex_shader, fragment_shader)
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

    // Enables a magic uniform value
    pub fn enable_uniform(mut self, uniform: MagicUniform) -> Self {
        self.magic_uniforms = self.magic_uniforms | uniform;
        self
    }

    // Checks which magic uniforms are enabled and then sets them accordingly
    fn update_magic_uniforms(&self, vars: &FrameState) -> Result<(), ProgramError> {
        if self.magic_uniforms.contains(MagicUniform::TIME) {
            self.set_uniform("time", GL1F(vars.time))?;
        }
        if self.magic_uniforms.contains(MagicUniform::RESOLUTION) {
            if let Some((x, y)) = vars.resolution {
                self.set_uniform("resolution", GL2F(x, y))?;
            }
        }
        Ok(())
    }

    // Create a new, or edit an existing, VAO
    pub fn vao_from_obj<S>(&mut self, name: S, obj: &Obj) -> Result<&mut Self, ProgramError>
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

    // Sets a uniform variable at the location
    pub fn set_uniform<S, Type>(&self, name: S, mut value: Type) -> Result<(), ProgramError>
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
    fn get_uniform_location<S>(&self, name: S) -> Result<GLint, ProgramError>
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

    // Updates the magic uniforms, draws every VAO in order
    pub fn draw(&self, frame_events: &FrameState) -> Result<(), ProgramError> {
        self.update_magic_uniforms(&frame_events)?;
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
        Ok(())
    }
    // Create a new OpenGL Program
    // pub fn builder() -> GLProgramBuilder<NoVS, NoFS> {
    //     let id;
    //     unsafe {
    //         id = gl::CreateProgram();
    //     }
    //     GLProgramBuilder {
    //         id,
    //         vertex_shader: NoVS,
    //         fragment_shader: NoFS,
    //     }
    // }
}

// Easy conversion from builder to program
// impl<'a, V, F> From<GLProgramBuilder<V, F>> for GLProgram<V, F> {
//     fn from(builder: GLProgramBuilder<V, F>) -> Self {
//         let GLProgramBuilder {
//             id,
//             vertex_shader,
//             fragment_shader,
//         } = builder;
//
//         GLProgram {
//             id,
//             vertex_shader,
//             fragment_shader,
//             magic_uniforms: MagicUniform::NONE,
//             vaos: HashMap::new(),
//         }
//     }
// }

// Helper function that checks if linking the shaders to the program was a success
pub(crate) fn link_shaders_success(program_id: GLuint) -> Result<(), ProgramError> {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            // Determine the log's length
            let mut length = 0 as GLint;
            gl::GetShaderiv(program_id, gl::INFO_LOG_LENGTH, &mut length);
            let log_length: usize = length.try_into().map_err(|_| {
                ProgramError::Linking(String::from("Couldn't determine length of error log."))
            })?;

            // Set up a buffer to receive the log
            let mut error_log = Vec::<u8>::with_capacity(log_length);
            if log_length > 0 {
                error_log.set_len(log_length - 1);
            } // Don't read the NULL terminator

            gl::GetProgramInfoLog(
                program_id,
                512,
                std::ptr::null_mut(),
                error_log.as_mut_ptr() as *mut GLchar,
            );

            // Return the error log and exit
            Err(ProgramError::Linking(
                std::str::from_utf8(&error_log).unwrap().into(),
            ))
        } else {
            Ok(())
        }
    }
}
