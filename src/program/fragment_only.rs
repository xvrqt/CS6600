use super::GLProgram;

use super::GLDraw;
use crate::obj::Obj;
use crate::program::camera::ArcBallCamera;
use crate::program::Camera;
use crate::program::ProgramError;
use crate::program::{LightColor, LightSource};
use crate::shader::ShaderPipeline;
use crate::types::*;
use crate::uniform::MagicUniform;
use crate::uniform::GL3FV;
use crate::vao::VAO;
use crate::window::FrameState;
use crate::Position;
use gl::types::*;
use glfw::Context;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use ultraviolet::mat::{Mat3, Mat4};
use ultraviolet::vec::Vec3;
type Result<T> = std::result::Result<T, ProgramError>;

pub struct FragmentOnly {
    uniforms: MagicUniform,
}

impl FragmentOnly {
    pub(crate) fn new() -> Self {
        FragmentOnly::default()
    }

    pub(crate) const TRIANGLE: ([f32; 9], GLsizeiptr, GLint, GLuint) = (
        // Vertices
        [-1.0, -1.0, 0.0, 3.0, -1.0, 0.0, -1.0, 3.0, 0.0],
        // Size
        (std::mem::size_of::<f32>() * 9) as GLsizeiptr,
        // Stride
        (std::mem::size_of::<f32>() * 3) as GLint,
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
            self.set_uniform("time", GL1F(vars.time))?;
        }
        if self.data.uniforms.contains(MagicUniform::RESOLUTION) {
            if let Some((x, y)) = vars.resolution {
                self.set_uniform("resolution", GL2F(x, y))?;
            }
        }
        Ok(())
    }
}

impl<'a> GLDraw for GLProgram<'a, FragmentOnly> {
    fn draw(&mut self) -> Result<()> {
        // Set OpenGL State for this Program
        unsafe {
            gl::UseProgram(self.id);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Sets up 'self.context.frame_state' based on polled events
        self.context.glfw.poll_events();
        self.context.process_events();

        // Check if we should exit
        if self.context.window.should_close() {
            return Err(ProgramError::End);
        }

        // Update any magic uniform variables
        self.update_magic_uniforms(&self.context.frame_state)?;

        // Draw our single triangle
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        self.context.window.swap_buffers();

        Ok(())
    }
}
