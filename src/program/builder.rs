// For building a certain type of GLProgram
use gl;
use ultraviolet::Vec3;

use super::{
    blinn_phong::BlinnPhong, camera::ArcBallCamera, CustomShader, GLProgram, GLWindow, ProgramError,
};
type Result<T> = std::result::Result<T, ProgramError>;
// Shaders determine, in large part, the type of the Builder
use crate::shader::{
    FragmentShader, GeometryShader, Shader, ShaderPipeline, TesselationShader, VertexShader,
};

// Convenient use of special types that work well with OpenGL
use super::fragment_only::FragmentOnly;
use gl::types::*;

// Dummy types to create a builder system for GLProgram
#[derive(Debug)]
pub struct NoWindow;
#[derive(Debug)]
pub struct NoVS;
#[derive(Debug)]
pub struct NoFS;

#[derive(Debug)]
pub struct GLProgramBuilder<'a, W, V, F> {
    window: W,
    vertex_shader: V,
    fragment_shader: F,
    geometry_shader: Option<Shader<'a, GeometryShader>>,
    tessellation_shader: Option<Shader<'a, TesselationShader>>,
}

impl<'a> GLProgram<'a, CustomShader> {
    // Returns a builder for a ne custom shader OpenGL program
    pub fn new() -> GLProgramBuilder<'a, NoWindow, NoVS, NoFS> {
        GLProgramBuilder {
            window: NoWindow,
            vertex_shader: NoVS,
            fragment_shader: NoFS,
            geometry_shader: None,
            tessellation_shader: None,
        }
    }

    // Shortcut to creating a Fragment Only GLProgram (think ShaderToy)
    pub fn fragment_only<S>(fragment_shader_source: &'a S) -> Result<GLProgram<'a, FragmentOnly>>
    where
        S: AsRef<str>,
    {
        // Setup the GLProgram Struct
        let (id, context) = initialize()?;
        let vs = Shader::<VertexShader>::fragment_only()?;
        let fragment_shader: Shader<'a, FragmentShader> =
            Shader::<'a, FragmentShader>::new(fragment_shader_source.as_ref())?;
        let shaders = ShaderPipeline::new(id, vs, fragment_shader, None, None)?;
        let data = FragmentOnly::new();

        // Initialize the vertices for the vertex shader since they will never change
        let (vertices, size, stride, location) = FragmentOnly::TRIANGLE;
        let ptr = vertices.as_ptr() as *const std::ffi::c_void;

        unsafe {
            // Use the program context we just created
            gl::UseProgram(id);
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            println!("vao: {}", vao);

            // Buffer the vertices to the GPU
            let mut buffer_id: GLuint = 0;
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);
            println!("buffer_id: {}", buffer_id);

            // Create our sole Vetex Attray Object
            gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(location);
        }

        Ok(GLProgram {
            id,
            context,
            shaders,
            data,
        })
    }

    // Shortcut to creating a GLProgram that users Blinn-Phong Shading
    pub fn phong() -> Result<GLProgram<'a, BlinnPhong>> {
        let (id, context) = initialize()?;
        let vs = Shader::<VertexShader>::blinn_phong()?;
        let fs = Shader::<FragmentShader>::phong()?;
        let shaders = ShaderPipeline::new(id, vs, fs, None, None)?;
        let data = BlinnPhong::new();
        Ok(GLProgram {
            id,
            context,
            shaders,
            data,
        })
    }
    pub fn blinn() -> Result<GLProgram<'a, BlinnPhong>> {
        let (id, context) = initialize()?;
        let vs = Shader::<VertexShader>::blinn_phong()?;
        let fs = Shader::<FragmentShader>::blinn()?;
        let shaders = ShaderPipeline::new(id, vs, fs, None, None)?;
        let data = BlinnPhong::new();
        Ok(GLProgram {
            id,
            context,
            shaders,
            data,
        })
    }
}

// Create a new window, and OpenGL context.
impl<'a, NoWindow, V, F> GLProgramBuilder<'a, NoWindow, V, F> {
    // User provides the window
    pub fn use_window(self, window: GLWindow) -> Result<GLProgramBuilder<'a, GLWindow, V, F>> {
        let GLProgramBuilder {
            vertex_shader,
            fragment_shader,
            geometry_shader,
            tessellation_shader,
            ..
        } = self;
        Ok(GLProgramBuilder {
            window,
            vertex_shader,
            fragment_shader,
            geometry_shader,
            tessellation_shader,
        })
    }

    // Use the default window settings
    pub fn use_default_window(self) -> Result<GLProgramBuilder<'a, GLWindow, V, F>> {
        self.use_window(GLWindow::default()?)
    }
}

// Allow the user to attach custom shaders to different parts of the graphics pipeline.
impl<'a, W, NoVS, NoFS> GLProgramBuilder<'a, W, NoVS, NoFS> {
    pub fn with_shaders(
        self,
        vertex_shader: Shader<'a, VertexShader>,
        fragment_shader: Shader<'a, FragmentShader>,
        geometry_shader: Option<Shader<'a, GeometryShader>>,
        tessellation_shader: Option<Shader<'a, TesselationShader>>,
    ) -> GLProgramBuilder<'a, W, Shader<'a, VertexShader>, Shader<'a, FragmentShader>> {
        let gs = geometry_shader;
        let ts = tessellation_shader;
        let GLProgramBuilder {
            window,
            geometry_shader,
            tessellation_shader,
            ..
        } = self;
        GLProgramBuilder {
            window,
            vertex_shader,
            fragment_shader,
            geometry_shader: gs.or(geometry_shader),
            tessellation_shader: ts.or(tessellation_shader),
        }
    }
}

// Every constructor creates a new program ID, creates a window + context, and initializes the OpenGL pointers
#[inline(always)]
fn initialize() -> Result<(GLuint, GLWindow)> {
    // Load pointers, using the context
    let mut context = GLWindow::default()?;
    gl::load_with(|symbol| context.window.get_proc_address(symbol) as *const _);
    let id = create_program_id();
    Ok((id, context))
}

#[inline(always)]
fn create_program_id() -> GLuint {
    unsafe { gl::CreateProgram() }
}
