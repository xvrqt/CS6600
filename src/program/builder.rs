use super::{BlinnPhong, CustomShader, FragmentOnly, GLProgram, GLWindow, ProgramError};

// All GLPrograms have a ShaderPipline which is composed of at least a VertexShader and
// FragmentShader and may optionally have additional types of shaders
use crate::shader::{
    FragmentShader, GeometryShader, Shader, ShaderPipeline, TesselationShader, VertexShader,
};

// // Convenience Error Type Alias
type Result<T> = std::result::Result<T, ProgramError>;

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
        // Initialize a window, context and OpenGL program
        let (id, context) = initialize()?;
        // Initialize and link shaders to the program
        let vs = Shader::<VertexShader>::fragment_only()?;
        let fragment_shader: Shader<'a, FragmentShader> =
            Shader::<'a, FragmentShader>::new(fragment_shader_source.as_ref())?;
        let shaders = ShaderPipeline::new(id, vs, fragment_shader, None, None)?;
        // Initialize the sub-structure of this <Type> of GLProgram
        // This will also setup the OpenGL context with the vertex data necessary to trigger the
        // firing of every fragment.
        let data = FragmentOnly::new(id);

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
fn initialize() -> Result<(gl::types::GLuint, GLWindow)> {
    // Load pointers, using the context
    let mut context = GLWindow::default()?;
    gl::load_with(|symbol| context.window.get_proc_address(symbol) as *const _);
    let id = create_program_id();
    Ok((id, context))
}

#[inline(always)]
fn create_program_id() -> gl::types::GLuint {
    unsafe { gl::CreateProgram() }
}
