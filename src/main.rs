// My Libs
use cs6600::{load::load_obj, GLError, GLProgram};
use ultraviolet::vec::Vec3;

#[allow(non_snake_case)]
fn main() -> Result<(), GLError> {
    let mut program = GLProgram::blinn_phong()?;

    let obj = load_obj("./objs/teapot.obj")?;
    let _ = program.vao_from_obj("gay", &obj);
    // let _ = program.add_light(Vec3::new(5.0, 5.0, 5.0), Vec3::new(0.5, 0.5, 0.5));
    let _ = program.add_light(Vec3::new(0.0, 5.0, 5.0), Vec3::new(0.7, 0.0, 0.0));
    let _ = program.add_light(Vec3::new(5.0, 5.0, 0.0), Vec3::new(0.0, 0.7, 0.0));
    let _ = program.add_light(Vec3::new(5.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.7));
    let _ = program.ambient_light(Vec3::new(1.0, 1.0, 1.0), 0.1);

    while program.draw().is_ok() {}
    //
    //
    //     let frame_state = process_events(&glfw, &mut window, &events)?;
    //     let delta_t = frame_state.time - last_frame;
    //     last_frame = frame_state.time;
    //     let radius = 25.0;
    //     let cam_x = glfw.get_time().sin() * radius;
    //     let cam_z = glfw.get_time().cos() * radius;
    //
    //     let (pos_x, pos_y) = window.get_cursor_pos();
    //     let x_offset = (pos_x - mouse_last_x) * sensitivity;
    //     let y_offset = (mouse_last_y - pos_y) * sensitivity;
    //     mouse_last_x = pos_x;
    //     mouse_last_y = pos_y;
    //
    // }
    Ok(())
}
