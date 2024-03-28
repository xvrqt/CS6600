// My Libs
use cs6600::{load::load_obj, process_events, window, GLError, GLProgram};

// Window Creation + Control
use glfw::Context;
// Linear Algebra Crate
#[allow(unused_imports)]
use ultraviolet::{
    mat::{Mat3, Mat4},
    vec::{Vec2, Vec3, Vec4},
};

#[allow(non_snake_case)]
fn main() -> Result<(), GLError> {
    // GLFW lib handle, window handle, and event loop for that window handle
    let (mut glfw, mut window, events) = window::create_default()?;

    // Load function pointers from the user's linked OpenGL library
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let obj = load_obj("./src/wires.obj")?;

    // Use built-in Blinn-Phong Shader
    let mut program = GLProgram::blinn_phong_shading()?;
    program.point_camera_at_origin(Vec3::new(5.0, -5.0, 19.0));
    program.add_light(Vec3::new(5.0, -5.0, 19.0), Vec3::new(0.2, 0.2, 0.2))?;
    // program.add_light(Vec3::new(14.0, -1.0, -14.0), Vec3::new(0.5, 0.2, 0.9))?;
    let _ = program.vao_from_obj("gay", &obj);

    // In case we have more than one program, render all of them
    let render_queue = vec![program];
    while !window.should_close() {
        // Process events, and extract relevant program details
        let frame_state = process_events(&glfw, &mut window, &events)?;

        // RENDER
        for program in render_queue.iter() {
            program.draw(&frame_state)?;
        }

        // Show the buffer on screen, poll for new events, and start again
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}
