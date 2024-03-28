// My Libs
use cs6600::{
    load::load_obj,
    // Extracts useful variables from the window + event state each frame
    process_events,
    // For loading shaders of these types
    shader::{Fragment, Vertex},
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

    let obj = load_obj("./src/wires.obj")?;

    // Use built-in Blinn-Phong Shader
    let mut program = GLProgram::blinn_phong_shading()?;
    let _ = program.vao_from_obj("gay", &obj);

    // In case we have more than one program, render all of them
    let render_queue = vec![program];
    while !window.should_close() {
        // Process events, and extract relevant program details
        let frame_state = process_events(&glfw, &mut window, &events)?;

        // Generate perspective transform
        // Rotate the objects alond the X-Z Plane
        let mut rotation = ultraviolet::mat::Mat4::from_rotation_x(1.0 * frame_state.time / 5.0);
        rotation = rotation * ultraviolet::mat::Mat4::from_rotation_y(1.0 * frame_state.time / 5.0);
        // Pull the camera back a bit
        let camera =
            ultraviolet::mat::Mat4::from_translation(ultraviolet::vec::Vec3::new(0.0, 0.0, -50.0));
        // ultraviolet::mat::Mat4::look_at(ultraviolet::Vec3::new(0.0,0.0,1.0), ultraviolet::Vec3::new(0.0, 0.0, 0.0), ultraviolet::Vec3::new(0.0,1.0,0.0));
        // let side = 2.0;
        // let ortho =
        //     ultraviolet::projection::rh_yup::orthographic_gl(-side, side, -side, side, -side, side);
        // Modify for perspective
        let perspective = frame_state.perspective_matrix;
        let mvp = perspective * camera * rotation;

        // Calculate the model-view transform matrix
        let mv = camera * rotation;

        // Calculate the normal model-view transform matrix
        let mut mvn: ultraviolet::mat::Mat3 = mv.truncate();
        mvn.inverse();
        mvn.transpose();

        // RENDER
        for program in render_queue.iter() {
            program.set_uniform("mvp", mvp)?;
            program.set_uniform("mvn", mvn)?;
            program.set_uniform("mv", mv)?;
            program.draw(&frame_state)?;
        }

        // Show the buffer on screen, poll for new events, and start again
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}
