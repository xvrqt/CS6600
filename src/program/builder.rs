// For building a certain type of GLProgram
use gl;

use super::{camera::ArcBallCamera, BlinnPhong, CustomShader, GLProgram, GLWindow, ProgramError};
type Result<T> = std::result::Result<T, ProgramError>;
// Shaders determine, in large part, the type of the Builder
use crate::shader::{
    FragmentShader, GeometryShader, Shader, ShaderPipeline, TesselationShader, VertexShader,
};

// Convenient use of special types that work well with OpenGL
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

    // Shortcut to creating a GLProgram that users Blinn-Phong Shading
    pub fn phong() -> Result<GLProgram<'a, BlinnPhong>> {
        // Load the function pointers
        let mut context = GLWindow::default()?;
        gl::load_with(|symbol| context.window.get_proc_address(symbol) as *const _);
        let id = create_program_id();
        let vs = Shader::<VertexShader>::blinn_phong()?;
        let fs = Shader::<FragmentShader>::phong()?;
        let shaders = ShaderPipeline::new(id, vs, fs, None, None)?;
        Ok(GLProgram {
            id,
            context,
            shaders,
            camera: Box::new(ArcBallCamera::new()),
            lights: Vec::new(),
            lights_buffer: None,
            vaos: std::collections::HashMap::new(),
            _pd: std::marker::PhantomData::<BlinnPhong>,
        })
    }
    pub fn blinn() -> Result<GLProgram<'a, BlinnPhong>> {
        // Load the function pointers
        let mut context = GLWindow::default()?;
        gl::load_with(|symbol| context.window.get_proc_address(symbol) as *const _);
        let id = create_program_id();
        let vs = Shader::<VertexShader>::blinn_phong()?;
        let fs = Shader::<FragmentShader>::blinn()?;
        let shaders = ShaderPipeline::new(id, vs, fs, None, None)?;
        Ok(GLProgram {
            id,
            context,
            shaders,
            camera: Box::new(ArcBallCamera::new()),
            lights: Vec::new(),
            lights_buffer: None,
            vaos: std::collections::HashMap::new(),
            _pd: std::marker::PhantomData::<BlinnPhong>,
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

// If the caller has an OpenGL Context/Window, and at least a Vertex and Fragment shader then allow
// them to create a GLProgram from the builder.
impl<'a> GLProgramBuilder<'a, GLWindow, Shader<'a, VertexShader>, Shader<'a, FragmentShader>> {
    fn compile(mut self) -> Result<GLProgram<'a, CustomShader>> {
        // Load the function pointers
        gl::load_with(|symbol| self.window.window.get_proc_address(symbol) as *const _);
        let id = create_program_id();
        let shaders = ShaderPipeline::new(
            id,
            self.vertex_shader,
            self.fragment_shader,
            self.geometry_shader,
            self.tessellation_shader,
        )?;
        Ok(GLProgram {
            id,
            context: self.window,
            shaders,
            camera: Box::new(ArcBallCamera::new()),
            lights: Vec::new(),
            lights_buffer: None,
            vaos: std::collections::HashMap::new(),
            _pd: std::marker::PhantomData::<CustomShader>,
        })
    }
}

// Keeping things DRY
fn create_program_id() -> GLuint {
    unsafe { gl::CreateProgram() }
}
