use super::mesh::Mesh;
use super::mesh::LOADED;
use super::mesh::UNLOADED;
use super::GLProgram;
use crate::load_mesh;
use crate::program::camera::ArcBallCamera;
use crate::program::Camera;
use crate::program::ProgramError;
use crate::program::{LightColor, LightSource};
use crate::types::*;
use crate::vao::VAO;
use crate::Position;
use gl::types::*;
use glfw::Context;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use ultraviolet::mat::{Mat3, Mat4};
type Result<T> = std::result::Result<T, ProgramError>;

pub struct BlinnPhong {
    camera: Box<dyn Camera>,
    lights: Vec<LightSource>,
    lights_buffer: Option<GLuint>,
    meshes: HashMap<String, Mesh<LOADED>>,
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
            meshes: HashMap::new(),
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

    pub fn add_mesh_from_file<S, P>(
        &mut self,
        mesh_name: S,
        path: P,
        draw_style: GLuint,
    ) -> Result<()>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        let mesh = load_mesh(path)?;
        let mut mesh = mesh.load(self.id)?;
        mesh.state.0.draw_style = draw_style;

        self.data
            .meshes
            // TODO: Error on collision ?
            .insert(mesh_name.as_ref().to_string(), mesh);
        Ok(())
    }

    pub fn add_mesh<S>(&mut self, mesh_name: S, mesh: Mesh<UNLOADED>) -> Result<()>
    where
        S: AsRef<str>,
    {
        let mesh = mesh.load(self.id)?;
        self.data
            .meshes
            .insert(mesh_name.as_ref().to_string(), mesh);
        Ok(())
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
            gl::PointSize(3.0);
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

        for mesh in self.data.meshes.values() {
            let vao = &mesh.state.0;
            if vao.enabled {
                unsafe {
                    gl::BindVertexArray(vao.id);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vao.ele_buffer);
                    gl::DrawElements(
                        vao.draw_style,
                        vao.draw_count,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
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
