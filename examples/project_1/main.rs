// My Libs
use cs6600::{
    // Trait all GLPrograms conform to that allows the `.draw()` function
    program::GLDraw,
    // Conveninece function for loading shader sources
    shader::load_shader,
    // Used to enable the automagical setting of common uniform variables
    uniform::MagicUniform,
    // Crate Error Type
    GLError,
    // The OpenGL Program we're configuring
    GLProgram,
};

fn main() -> Result<(), GLError> {
    // Load shader source from file
    let fragment_shader = load_shader("./examples/project_1/p1.frag")?;

    // Link Shader to Program
    let mut program = GLProgram::fragment_only(&fragment_shader)?
        .enable_uniform(MagicUniform::RESOLUTION)
        .enable_uniform(MagicUniform::TIME);

    // Render loop
    while program.draw().is_ok() {}
    Ok(())
}
