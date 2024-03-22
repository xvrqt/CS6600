// My Libs
use cs6600::{
    // Load shaders, objects, textures from files
    load::load_shader,
    // Extracts useful variables from the window + event state each frame
    process_events,
    // For loading shaders of these types
    shader::{Fragment, Vertex},
    // Shortcut to use my uniform and attribute types
    types::*,
    // Used to enable the automagical setting of common uniform variable
    uniform::MagicUniform,
    // Window, main() error type, OpenGL programs, and Shaders
    window,
    GLError,
    GLProgram,
    Shader,
};

// Window Creation + Control
use glfw::Context;
// Linear Algebra Crate
use ultraviolet;

#[allow(non_snake_case)]
fn main() -> Result<(), GLError> {
    // GLFW lib handle, window handle, and event loop for that window handle
    let (mut glfw, mut window, events) = window::create_default()?;

    // Load function pointers from the user's linked OpenGL library
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Compile Shaders
    let vs_source = load_shader("examples/project_1/p1.vert")?;
    let vertex_shader = Shader::<Vertex>::new(&vs_source)?;
    let fs_source = load_shader("examples/project_1/p1.frag")?;
    let fragment_shader = Shader::<Fragment>::new(&fs_source)?;

    // Link Shaders to Program
    let mut program = GLProgram::builder()
        .attach_vertex_shader(vertex_shader)
        .attach_fragment_shader(fragment_shader)
        .link_shaders()?
        .enable_uniform(MagicUniform::TIME) // Will set the float 'time' as a uniform every call
        .enable_uniform(MagicUniform::RESOLUTION); // Will pass the 'resolution' as a vec2

    // Generate Object Data
    let triangle = GL3FV(vec![
        GL3F(-0.5, -0.5, 0.0),
        GL3F(0.5, -0.5, 0.0),
        GL3F(0.0, 0.5, 0.0),
        GL3F(-0.5, -0.5, 0.5),
        GL3F(0.5, -0.5, 0.5),
        GL3F(0.0, 0.5, 0.5),
        GL3F(-0.5, -0.5, -0.5),
        GL3F(0.5, -0.5, -0.5),
        GL3F(0.0, 0.5, -0.5),
    ]);

    let colors = GL3FV(vec![
        GL3F(1.0, 0.0, 0.0),
        GL3F(0.0, 1.0, 0.0),
        GL3F(0.0, 0.0, 1.0),
        GL3F(0.0, 0.0, 1.0),
        GL3F(1.0, 0.0, 0.0),
        GL3F(0.0, 1.0, 0.0),
        GL3F(0.0, 1.0, 0.0),
        GL3F(0.0, 0.0, 1.0),
        GL3F(1.0, 0.0, 0.0),
    ]);

    // Create a new object, and attach some data to it
    program
        .vao("triangle")
        .attribute("vertices", triangle)?
        .attribute("colors", colors)?;

    // In case we have more than one program, render all of them
    let render_queue = vec![program];
    while !window.should_close() {
        // Process events, and extract relevant program details
        let frame_state = process_events(&glfw, &mut window, &events)?;

        // Generate perspective transform
        // Rotate the objects alond the X-Z Plane
        let rotation = ultraviolet::mat::Mat4::from_rotation_y(frame_state.time);
        // Pull the camera back a bit
        let camera =
            ultraviolet::mat::Mat4::from_translation(ultraviolet::vec::Vec3::new(0.0, 0.0, -5.0));
        // Modify for perspective
        let perspective = frame_state.perspective_matrix;
        let mvp = perspective * camera * rotation;

        // RENDER
        for program in render_queue.iter() {
            program.set_uniform("mvp", mvp)?;
            program.draw(&frame_state)?;
        }

        // Show the buffer on screen, poll for new events, and start again
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}
