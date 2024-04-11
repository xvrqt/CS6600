// My Libs
use cs6600::{GLError, GLProgram, GLStatus, LightColor, Mesh, Position};

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use ultraviolet::mat::Mat4;
use ultraviolet::vec::Vec3;

fn main() -> std::result::Result<(), GLError> {
    // let mut rng = SmallRng::from_entropy(); let tiny_teapot = Mat4::from_scale(0.5);
    // let transform = Mat4::from_translation(Vec3::new(10.0, 10.0, 10.0));
    // let moved_teapot = Mat4::from_translation(ultraviolet::vec::Vec3::new(10.0, 10.0, 10.0));

    let mut program = GLProgram::blinn()?;
    let teapot = Mesh::parse("./objs/teapot.obj")?;
    // TODO: Return some sort of reference to this instead of a string ?
    program.attach_mesh(teapot)?;
    program.create_object("teapot", "teapot", Mat4::identity());

    // program.new_object("tiny_teapot", teapot.clone(), tiny_teapot)?;
    // program.new_object("moved_teapot", teapot.clone(), moved_teapot)?;
    // for i in 0..9998 {
    //     let mesh = teapot.clone();
    //
    //     let x = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
    //     let y = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
    //     let z = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
    //     let translate = ultraviolet::Mat4::from_translation(ultraviolet::vec::Vec3::new(x, y, z));
    //     let scale = f32::powf(
    //         0.5,
    //         ((rng.next_u32() as f32 / std::u32::MAX as f32) * 5.0) - 0.5,
    //     );
    //     let scale = ultraviolet::mat::Mat4::from_scale(scale);
    //     let r = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
    //     let y = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
    //     let p = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
    //     let rotate = ultraviolet::mat::Mat4::from_euler_angles(r, p, y);
    //     let transformed_teapot = translate * scale * rotate;
    //     program.new_object(i.to_string().as_str(), mesh, transformed_teapot)?;
    // }
    // let monkey = Mesh::parse("./objs/monkey.obj")?;
    // program.new_object("monkey", monkey)?;

    let ambient_light = LightColor::new(1.0, 1.0, 1.0, 0.01);
    let red_light = LightColor::new(1.0, 0.0, 0.0, 1.0);
    let green_light = LightColor::new(0.0, 1.0, 0.0, 1.0);
    let blue_light = LightColor::new(0.0, 0.0, 1.0, 1.0);
    let location_1 = Position::new(-50.0, 0.0, 0.0);
    let location_2 = Position::new(0.0, -50.0, 0.0);
    let location_3 = Position::new(50.0, 0.0, 0.0);

    program.add_light(&location_1, &red_light)?;
    program.add_light(&location_2, &green_light)?;
    program.add_light(&location_3, &blue_light)?;
    program.ambient_light(&ambient_light)?;

    // Ok(program.render()?)
    let result = loop {
        match program.render() {
            Ok(_) => (),
            Err(error) => break error,
        }
    };
    Err(result.into())
}
