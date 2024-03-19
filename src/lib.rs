// Compiling shaders into OpenGL programs
pub mod shaders;
// Opening a window with an OpenGL conteext
pub mod window;

// Our Errors will all roll up into this error type for easy handling
pub enum GLError {
    Program,
    Shader,
}
