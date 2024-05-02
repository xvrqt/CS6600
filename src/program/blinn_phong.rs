use super::mesh::{Attached, Mesh, Unattached};
use super::GLDraw;
use super::GLProgram;
use crate::program::camera::ArcBallCamera;
use crate::program::scene_object::SceneObject;
use crate::program::Camera;
use crate::program::ProgramError;
use crate::program::{LightColor, LightSource};
use crate::types::*;
use crate::Position;

use gl::types::*;
use glfw::Context;

use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::{Rc, Weak};
type Result<T> = std::result::Result<T, ProgramError>;

use ultraviolet::mat::{Mat3, Mat4};
use ultraviolet::vec::Vec4;

pub struct BlinnPhong {
    camera: Box<dyn Camera>,
    lights: Vec<LightSource>,
    lights_buffer: Option<GLuint>,
    scene_objects: HashMap<String, Rc<SceneObject>>,
    meshes: HashMap<String, Mesh<Attached>>,
    stdout: std::io::StdoutLock<'static>,
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
            scene_objects: HashMap::new(),
            meshes: HashMap::new(),
            stdout: std::io::stdout().lock(),
        }
    }
}

// Used to create a GLProgram
impl<'a> GLProgram<'a, BlinnPhong> {
    // Add a mesh to the scene
    pub fn attach_mesh(&mut self, mesh: Mesh<Unattached>) -> Result<()> {
        let key = mesh.name.clone();
        let value = mesh.attach(self.id)?;
        self.data.meshes.insert(key, value);
        Ok(())
    }

    // Instantiates a mesh as a new, named, object. Transform is a Mat4 representing the affine
    // transformation of the object in world space.
    pub fn create_object<S>(&mut self, name: S, mesh_name: S, transform: Mat4) -> ()
    where
        S: AsRef<str>,
    {
        // Create a new SceneObject
        let object = SceneObject::new(transform);
        let object = Rc::new(object);
        let mesh_object = Rc::downgrade(&object);

        // Look up the mesh
        let key = mesh_name.as_ref();
        let mesh = self.data.meshes.get_mut(key).unwrap();

        // Insert a weak reference to the object into the Mesh's own storage
        mesh.data.objects.push(mesh_object);

        // Insert the object into GLProgram's own map
        let key = name.as_ref().to_string();
        let value = object;
        self.data.scene_objects.insert(key, value);
    }

    // Adds a light to the scene
    pub fn add_light(&mut self, position: &Position, color: &LightColor) -> Result<()> {
        // Transform into homogeneous 3D coordinates, and add full opacity to the color value
        let light = LightSource::new(color, position);
        self.data.lights.push(light);
        // Since we're buffering to the GPU we don't want extra mememory at the end
        // Not sure this matters at all lmao, might be triggering extra allocations per light add
        // self.data.lights.shrink_to_fit();

        // Get location of the lights on the GPU, and length of the array CPU side
        let block_id = self.get_uniform_block_index("Lights")?;
        let num_lights: GLuint = self.data.lights.len() as u32;
        println!("Lights Block Index: {}", block_id);

        // Rebind the lights (This is the layout in the shader code)
        let binding_point = 1 as GLuint;

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
            let size = (num_lights * std::mem::size_of::<LightSource>() as u32) as GLsizeiptr;
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id);
            gl::BufferData(gl::UNIFORM_BUFFER, size, ptr, gl::DYNAMIC_DRAW);
        }
        // Bind the buffer and the block to the same place
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, buffer_id);
        }
        self.update_uniform("num_lights", &num_lights)?;
        Ok(())
    }

    // Set the ambient light for the scene
    pub fn ambient_light(&mut self, color: &LightColor) -> Result<Weak<dyn UpdateUniform>> {
        self.create_uniform("ambient_light_color", &color.clone().to_vec4())
    }

    pub(crate) fn initialize(&mut self) -> Result<()> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::CullFace(gl::BACK);
            gl::PointSize(3.0);
        }

        // Create uniforms
        let zero_vector = Vec4::default();
        let identity_matrix = Mat4::identity();
        self.create_uniform("ambient_light_color", &zero_vector)?;
        self.create_uniform("view_projection_matrix", &identity_matrix)?;
        self.create_uniform("camera_position", &identity_matrix)?;
        self.create_uniform("num_lights", &0)?;

        self.context
            .glfw
            .set_swap_interval(glfw::SwapInterval::None);
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        // Set OpenGL State for this Program
        unsafe {
            gl::UseProgram(self.id);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Set uniforms for vertex view-perspective transform, and camera position
        if let Some(vpm) = self.data.camera.view_projection_matrix() {
            self.update_uniform("view_projection_matrix", &vpm)?;
            let camera_position = self.data.camera.position();
            self.update_uniform("camera_position", &camera_position)?;
        }

        for mesh in self.data.meshes.values_mut() {
            mesh.draw()?;
        }

        Ok(())
    }

    // Draws the next frame of the program
    pub fn render(&mut self) -> Result<()> {
        self.context.glfw.poll_events();
        // Sets up 'self.context.frame_state' based on polled events
        self.context.process_events();
        // Check if we should exit
        if self.context.window.should_close() {
            return Err(ProgramError::End);
        }

        // Update our camera based off of keyboard input
        // TODO: Generate the matrices in here, and only return them if they have changed
        self.data
            .camera
            .update(&mut self.context.frame_state.camera_events);

        self.draw()?;

        // FPS / Frame Interval Counter
        if self.context.frame_state.frame % 60 == 0 {
            let dt_60 = self.context.frame_state.delta_t_60.as_secs_f64();
            let dt = self.context.frame_state.delta_t.as_secs_f64();
            write!(
                self.data.stdout,
                "frame: {}\tinterval: {:.4}ms\tfps: {:.2}\r",
                self.context.frame_state.frame,
                dt * 1000.0,
                60.0 / dt_60,
            )
            .unwrap();
            self.data.stdout.flush().unwrap();
        }

        self.context.window.swap_buffers();
        Ok(())
    }
}
