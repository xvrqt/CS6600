use super::GLProgram;
use crate::obj::Obj;
use crate::program::camera::ArcBallCamera;
use crate::program::Camera;
use crate::program::ProgramError;
use crate::program::{LightColor, LightSource};
use crate::shader::ShaderPipeline;
use crate::types::*;
use crate::vao::VAO;
use crate::Position;
use gl::types::*;
use glfw::Context;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use ultraviolet::mat::{Mat3, Mat4};
type Result<T> = std::result::Result<T, ProgramError>;

pub struct BlinnPhong {
    camera: Box<dyn Camera>,
    lights: Vec<LightSource>,
    lights_buffer: Option<GLuint>,
    vaos: HashMap<String, VAO>,
}

impl BlinnPhong {
    pub(crate) fn new() -> Self {
        BlinnPhong::default()
    }
}

impl Default for BlinnPhong {
    fn default() -> Self {
        BlinnPhong {
            camera: Box::new(ArcBallCamera::new()),
            lights: Vec::new(),
            lights_buffer: None,
            vaos: HashMap::new(),
        }
    }
}

// Used to create a GLProgram
impl<'a> GLProgram<'a, BlinnPhong> {
    // Adds a light to the scene
    pub fn add_light(&mut self, position: &Position, color: &LightColor) -> Result<()> {
        // Transform into homogeneous 3D coordinates, and add full opacity to the color value
        let light = LightSource::new(color, position);
        self.data.lights.push(light);
        // Since we're buffering to the GPU we don't want extra mememory at the end
        self.data.lights.shrink_to_fit();

        // Get location of the lights on the GPU, and length of the array CPU side
        let block_id = self.get_uniform_block_index("Lights")?;
        let num_lights = self.data.lights.len();

        // Rebind the lights (This is the layout in the shader code)
        let binding_point = 0 as GLuint;

        // Initialize a Uniform Buffer for the lights, if we haven't alreaady
        let buffer_id = match self.data.lights_buffer {
            Some(id) => id,
            None => {
                let mut id = 0 as GLuint;
                unsafe {
                    gl::GenBuffers(1, &mut id);
                }
                self.data.lights_buffer = Some(id);
                id
            }
        };

        // Buffer the data
        unsafe {
            gl::UniformBlockBinding(self.id, block_id, binding_point);
            let ptr = self.data.lights.as_ptr() as *const std::ffi::c_void;
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
    pub fn ambient_light(&mut self, color: &LightColor) -> Result<()> {
        self.set_uniform("ambient_light_color", color.clone().to_vec4())
    }

    // Create a new, or edit an existing, VAO
    pub fn vao<S>(&mut self, name: S) -> &mut VAO
    where
        S: AsRef<str>,
    {
        // Return existing, or create a new VAO
        match self.data.vaos.entry(name.as_ref().to_string()) {
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
        let vao = match self.data.vaos.entry(name.as_ref().to_string()) {
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
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::BlendFunc(gl::ONE, gl::ONE);
            // gl::DepthFunc(gl::ALWAYS);
            gl::CullFace(gl::BACK);
        }

        // Sets up 'self.context.frame_state' based on polled events
        self.context.process_events();
        // Check if we should exit
        if self.context.window.should_close() {
            return Err(ProgramError::End);
        }

        // Update our camera based off of keyboard input
        self.data
            .camera
            .update(&mut self.context.frame_state.camera_events);

        // Set uniforms for Vertex perspective transform, and vertex and normal Model-View
        // Transform for inside the Blinn-Phong Vertex Shader
        let (mvp, mv, mvn) = self.generate_view_matrices();
        self.set_uniform("mvp", mvp)?;
        self.set_uniform("mvn", mvn)?;
        self.set_uniform("mv", mv)?;

        for vao in self.data.vaos.values() {
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
    // TODO: Move to Camera impl ?
    fn generate_view_matrices(&mut self) -> (Mat4, Mat4, Mat3) {
        // Camera-Space
        let mv = self.data.camera.view_matrix();
        // Canonical View Volume
        let mvp = self.data.camera.projection_matrix() * mv;
        // Remove scaling on normals, move into World-Space
        let mut mvn = mv.truncate();
        mvn.inverse();
        mvn.transpose();

        (mvp, mv, mvn)
    }
}
