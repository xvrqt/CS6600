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

    let (w, h) = window.get_size();
    let mut mouse_last_x = w as f64 / 2.0;
    let mut mouse_last_y = h as f64 / 2.0;
    let sensitivity = 0.001;
    window.set_cursor_pos(mouse_last_x, mouse_last_y);
    // Hide cursor, lock to window
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    // window.set_cursor_pos_callback(|window, x, y| {});
    // Load function pointers from the user's linked OpenGL library
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let obj = load_obj("./objs/axes.obj")?;

    // Use built-in Blinn-Phong Shader
    let mut program = GLProgram::blinn_phong_shading()?;
    program.point_camera_at_origin(Vec3::new(5.0, 5.0, 5.0));
    program.add_light(Vec3::new(5.0, 5.0, 5.0), Vec3::new(0.2, 0.2, 0.2))?;
    program.add_light(Vec3::new(5.0, 0.0, 5.0), Vec3::new(0.5, 0.9, 0.9))?;
    program.add_light(Vec3::new(-5.0, -5.0, 5.0), Vec3::new(0.5, 1.0, 0.5))?;
    program.ambient_light(Vec3::new(1.0, 1.0, 1.0), 0.55)?;
    // program.set_ortho(3.0, -10.0, 10.0);
    program.use_perspective();
    let _ = program.vao_from_obj("gay", &obj);

    // In case we have more than one program, render all of them
    let mut last_frame = 0.0;
    let mut render_queue = vec![program];
    while !window.should_close() {
        // Process events, and extract relevant program details
        let frame_state = process_events(&glfw, &mut window, &events)?;
        let delta_t = frame_state.time - last_frame;
        last_frame = frame_state.time;

        let (pos_x, pos_y) = window.get_cursor_pos();
        let x_offset = (pos_x - mouse_last_x) * sensitivity;
        let y_offset = (mouse_last_y - pos_y) * sensitivity;
        mouse_last_x = pos_x;
        mouse_last_y = pos_y;

        // RENDER
        for program in render_queue.iter_mut() {
            program
                .camera
                .update(&frame_state.camera_change, delta_t, x_offset, y_offset);
            program.draw(&frame_state)?;
        }

        // Show the buffer on screen, poll for new events, and start again
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}
