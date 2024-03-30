// #![allow(unused_imports)]
// #![allow(dead_code)]
// Import and Re-Export our Error Type
pub mod error;
pub use error::ProgramError;

pub mod camera;

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
use ultraviolet::mat::{Mat3, Mat4};
use ultraviolet::vec::{Vec2, Vec3, Vec4};

use self::camera::Camera;

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
pub struct GLProgram<V, F> {
    id: u32, // OpenGL keeps track of programs with integer IDs
    // We never read these again, but I can imagine a future where we would want to
    vertex_shader: V,
    fragment_shader: F,
    // List of uniforms we update automagically for the caller
    magic_uniforms: MagicUniform,
    // List of VAOs to render
    vaos: HashMap<String, VAO>,
    // Camera transformation
    pub camera: camera::Camera,
    // List of light sources
    lights: Option<Vec<LightSource>>,
    lights_buffer: Option<GLuint>,
    ortho: Projection,
    orthographic_projection_matrix: Mat4,
    perspective_projection_matrix: Mat4,
    use_perspective: bool,
}

pub struct NoVS;
pub struct NoFS;

const SIDE: f32 = 5.0;

#[derive(Debug)]
enum Projection {
    Ortho(f32, f32, f32, f32, f32, f32),
}

impl Projection {
    fn mat(&self) -> Mat4 {
        match self {
            Projection::Ortho(l, r, t, b, n, f) => Mat4::new(
                Vec4::new(2.0 / (r - l), 0.0, 0.0, 0.0),
                Vec4::new(0.0, 2.0 / (t - b), 0.0, 0.0),
                Vec4::new(0.0, 0.0, -2.0 / (f - n), 0.0),
                Vec4::new(
                    -(r + l) / (r - l),
                    -(t + b) / (t - b),
                    -(f + n) / (f - n),
                    1.0,
                ),
            ),
        }
    }
}

impl<'a> GLProgram<NoVS, NoFS> {
    fn default_projections() -> (Projection, Mat4) {
        let r = SIDE;
        let l = -SIDE;
        let t = SIDE;
        let b = -SIDE;
        let n = 4.0; // Near plane in z-axis
        let f = 2000.0; // Far plane in z-axis
        let ortho = Projection::Ortho(l, r, t, b, n, f);
        // let ortho = ultraviolet::projection::rh_yup::orthographic_gl(l, r, b, t, n, f);
        // let perspective = ultraviolet::projection::rh_yup::perspective_gl(1.0, 1.0, 0.1, 10000.0);
        let perspective = Mat4::new(
            // Vec4::new((2.0 * n) / (r - l), 0.0, 0.0, 0.0),
            // Vec4::new(0.0, (2.0 * n) / (t - b), 0.0, 0.0),
            // Vec4::new(
            //     (r + l) / (r - l),
            //     (t + b) / (t - b),
            //     -(f + n) / (f - n),
            //     -1.0,
            // ),
            // Vec4::new(0.0, 0.0, (-2.0 * f * n) / (f - n), 0.0),
            Vec4::new(n, 0.0, 0.0, 0.0),
            Vec4::new(0.0, n, 0.0, 0.0),
            Vec4::new(0.0, 0.0, n + f, -1.0),
            Vec4::new(0.0, 0.0, f * n, 0.0),
        );
        (ortho, perspective)
    }
    // Creates a new OpenGL Program using a built-in Blinn-Phong shader
    // Return type provides additional functions to more easily manage scenes
    pub fn blinn_phong_shading(
    ) -> Result<GLProgram<BlinnPhongVertexShader<'a>, BlinnPhongFragmentShader<'a>>, ProgramError>
    {
        // Compile Blinn-Phong Shaders
        let vertex_shader = Shader::<Vertex>::blinn_phong()?;
        let fragment_shader = Shader::<Fragment>::blinn_phong()?;
        let id;
        unsafe {
            id = gl::CreateProgram();

            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);

            gl::LinkProgram(id);
        }
        let (ortho, perspective_projection_matrix) = GLProgram::default_projections();
        let orthographic_projection_matrix = ortho.mat();

        // If it was a success, return the Blinn-Phong builder
        link_shaders_success(id).and_then(|_| {
            Ok(GLProgram {
                id,
                vertex_shader,
                fragment_shader,
                magic_uniforms: MagicUniform::NONE,
                vaos: HashMap::new(),
                camera: Camera::new(),
                lights: Some(Vec::new()),
                lights_buffer: None,
                ortho,
                orthographic_projection_matrix,
                perspective_projection_matrix,
                use_perspective: false,
            })
        })
    }

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

        let (ortho, perspective_projection_matrix) = GLProgram::default_projections();
        let orthographic_projection_matrix = ortho.mat();

        // If it was a success, return the Blinn-Phong builder
        link_shaders_success(id).and_then(|_| {
            Ok(GLProgram {
                id,
                vertex_shader,
                fragment_shader,
                magic_uniforms: MagicUniform::NONE,
                vaos: HashMap::new(),
                camera: Camera::new(),
                lights: None,
                lights_buffer: None,
                ortho,
                orthographic_projection_matrix,
                perspective_projection_matrix,
                use_perspective: false,
            })
        })
    }
}

const ORIGINM4: Mat4 = Mat4::new(ORIGINV4, ORIGINV4, ORIGINV4, Vec4::new(0.0, 0.0, 0.0, 1.0));
const ORIGINV3: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ORIGINV4: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
const Y_UNIT_V3: Vec3 = Vec3::new(0.0, 1.0, 0.0);

// Used to create a GLProgram
impl<'a> GLProgram<BlinnPhongVertexShader<'a>, BlinnPhongFragmentShader<'a>> {
    pub fn set_ortho(&mut self, side: f32, near_clip: f32, far_clip: f32) -> () {
        self.ortho = Projection::Ortho(-side, side, side, -side, near_clip, far_clip);
        self.orthographic_projection_matrix = self.ortho.mat();
    }

    pub fn use_perspective(&mut self) -> () {
        self.use_perspective = true;
    }
    // Sets the camera to the selected position, it always faces the origin
    pub fn point_camera_at_origin(&mut self, position: Vec3) -> () {
        self.camera.matrix = ultraviolet::mat::Mat4::look_at(position, ORIGINV3, Y_UNIT_V3);
    }

    // Adds a light to the scene
    // TODO: This function is disgusting, because we shouldn't need to unwrap the Option, we
    // will always have the Vec; lots of immutable/mutable borrow hacks. We also need to check that
    // we don't have over 100 light sources and throw an error.
    pub fn add_light(&mut self, position: Vec3, color: Vec3) -> Result<(), ProgramError> {
        let block_id = self.get_uniform_block_index("Lights")?;
        let mut num_lights = 0;

        if let Some(lights) = self.lights.as_mut() {
            lights.shrink_to_fit();
            let light = LightSource {
                color: Vec4::new(color.x, color.y, color.z, 1.0),
                position: Vec4::new(position.x, position.y, position.z, 1.0),
            };
            lights.push(light);
            num_lights = lights.len();

            // Rebind the lights
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
                let ptr = lights.as_ptr() as *const std::ffi::c_void;
                let size = (lights.len() * std::mem::size_of::<LightSource>()) as GLsizeiptr;
                gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id);
                gl::BufferData(gl::UNIFORM_BUFFER, size, ptr, gl::DYNAMIC_DRAW);
            }
            // Bind the buffer and the block to the same place
            unsafe {
                gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, buffer_id);
            }
        }
        self.set_uniform("num_lights", GL1U(num_lights as GLuint))?;
        Ok(())
    }

    // Set the ambient light for the scene
    pub fn ambient_light(&mut self, color: Vec3, intensity: GLfloat) -> Result<(), ProgramError> {
        let color = GL4F(color.x, color.y, color.z, 1.0);
        let intensity = intensity.clamp(0.0, 1.0);
        let intensity = GL1F(intensity);
        self.set_uniform("ambient_light_color", color)?;
        self.set_uniform("ambient_intensity", intensity)
    }

    // Sets the bounding box for orthographic projection
    pub fn set_scene_bounding_box() -> () {}

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

    fn get_uniform_block_index<S>(&self, name: S) -> Result<GLuint, ProgramError>
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

    // Updates the magic uniforms, draws every VAO in order
    pub fn draw(&mut self, frame_events: &FrameState) -> Result<(), ProgramError> {
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

        // Update projection matrices if the aspect ratio has changed
        if let Some(resolution) = frame_events.resolution {
            let aspect_ratio = resolution.0 / resolution.1;
            let Projection::Ortho(_, r, t, b, n, f) = self.ortho;
            let r = r * aspect_ratio;
            let l = -r;
            let new_ortho = Projection::Ortho(l, r, t, b, n, f);
            self.orthographic_projection_matrix = new_ortho.mat();
        }

        // Toggle projection if P was pressed
        if frame_events.toggle_projection {
            self.use_perspective = !self.use_perspective;
        }
        // Set uniforms for Vertex perspective transform, and vertex and normal Model-View
        // Transform for inside the Blinn-Phong Vertex Shader
        let (mvp, mv, mvn) = generate_view_matrices(
            self.perspective_projection_matrix,
            self.orthographic_projection_matrix,
            self.use_perspective,
            self.camera.matrix,
        );
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
        Ok(())
    }
}

// Help function to generate the view matrices used for setting vertex position and normals for
// Blinn-Phong Shaders
pub(crate) fn generate_view_matrices(
    perspective_matrix: Mat4,
    ortho_matrix: Mat4,
    use_perspective: bool,
    camera: Mat4,
) -> (Mat4, Mat4, Mat3) {
    // Set uniforms for camera position
    let mvp;
    if use_perspective {
        mvp = ortho_matrix * perspective_matrix * camera;
    } else {
        mvp = ortho_matrix * camera;
    }
    let mv = camera;
    let mut mvn: ultraviolet::mat::Mat3 = mv.truncate();
    mvn.inverse();
    mvn.transpose();

    (mvp, mv, mvn)
}

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
