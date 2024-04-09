// Trait that all GLProgram<Types> must implement
use super::{GLDraw, GLProgram, ProgramError};

// Custom types to play nice with OpenGL
use crate::types::*;
use gl::types::*;

// A way to easily implement, and update common per-frame GLSL Uniform values
use crate::uniform::MagicUniform;

// We need the context to implement GLDraw
use crate::window::FrameState;
use glfw::Context;

use std::ffi::c_void;
use std::mem::size_of;

// Convenience Error Type Alias
type Result<T> = std::result::Result<T, ProgramError>;

// GLProgram sub-type sub-structure
pub struct FragmentOnly {
    uniforms: MagicUniform,
}

impl FragmentOnly {
    // Generates the sub-structure AND also initializes OpenGL with the vertices it needs to
    // support it
    pub(crate) fn new(program_id: GLuint) -> Self {
        // Initialize the vertices for the vertex shader since they will never change
        let (ptr, size, stride, location) = FragmentOnly::VERTICES;

        unsafe {
            gl::UseProgram(program_id);
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Buffer the vertices to the GPU
            let mut buffer_id: GLuint = 0;
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);

            // Create our sole Vetex Attray Object
            gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(location);
        }
        FragmentOnly::default()
    }

    // Since we always draw the same triangle, just store it as a constant, along with it's
    // computed sizes, location, etc...
    pub(crate) const TRIANGLE: [f32; 9] = [-1.0, -1.0, 0.0, 3.0, -1.0, 0.0, -1.0, 3.0, 0.0];
    pub(crate) const VERTICES: (*const c_void, GLsizeiptr, GLint, GLuint) = (
        // Pointer
        FragmentOnly::TRIANGLE.as_ptr() as *const c_void,
        // Size
        (size_of::<f32>() * 9) as GLsizeiptr,
        // Stride
        (size_of::<f32>() * 3) as GLint,
        // Location (set in the GLSL source code)
        0,
    );
}

impl Default for FragmentOnly {
    fn default() -> Self {
        FragmentOnly {
            uniforms: MagicUniform::NONE,
        }
    }
}

// Used to create a GLProgram
impl<'a> GLProgram<'a, FragmentOnly> {
    // Enables a magic uniform value
    pub fn enable_uniform(mut self, uniform: MagicUniform) -> Self {
        self.data.uniforms = self.data.uniforms | uniform;
        self
    }

    // Checks which magic uniforms are enabled and then sets them accordingly
    fn update_magic_uniforms(&self, vars: &FrameState) -> Result<()> {
        if self.data.uniforms.contains(MagicUniform::TIME) {
            self.set_uniform("time", GL1F(vars.time.as_secs_f32()))?;
        }
        if self.data.uniforms.contains(MagicUniform::RESOLUTION) {
            if let Some((x, y)) = vars.resolution {
                self.set_uniform("resolution", GL2F(x, y))?;
            }
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        // Sets up 'self.context.frame_state' based on polled events
        self.context.glfw.poll_events();
        self.context.process_events();

        // Check if we should exit
        if self.context.window.should_close() {
            return Err(ProgramError::End);
        }
        // Update any magic uniform variables
        self.update_magic_uniforms(&self.context.frame_state)?;

        self.draw()?;
        self.context.window.swap_buffers();
        Ok(())
    }
}

impl<'a> GLDraw for GLProgram<'a, FragmentOnly> {
    fn draw(&self) -> Result<()> {
        // Set OpenGL State for this Program
        unsafe {
            gl::UseProgram(self.id);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw our single triangle
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        Ok(())
    }
}
