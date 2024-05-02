// My Libs
use cs6600::{GLError, GLProgram, LightColor, Mesh, Position};

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use ultraviolet::mat::Mat4;
use ultraviolet::vec::Vec3;

fn main() -> std::result::Result<(), GLError> {
    let mut rng = SmallRng::from_entropy();

    let mut program = GLProgram::blinn()?;

    let teapot = Mesh::parse("./objs/teapot.obj")?;
    program.attach_mesh(teapot)?;
    // TODO: Return some sort of reference to this instead of a string ?
    program.create_object("teapot", "teapot", Mat4::identity());

    let tiny_teapot = Mat4::from_scale(0.5);
    let moved_teapot = Mat4::from_translation(Vec3::new(10.0, 10.0, 10.0));
    program.create_object("tiny_teapot", "teapot", moved_teapot * tiny_teapot);
    // program.create_object("moved_teapot", "teapot", moved_teapot);

    // 10,000 TEAPOTS
    for i in 0..9998 {
        let x = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
        let y = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
        let z = ((rng.next_u32() as f32 / std::u32::MAX as f32) * 1000.0) - 500.0;
        let translate = Mat4::from_translation(Vec3::new(x, y, z));
        let scale = f32::powf(
            0.5,
            ((rng.next_u32() as f32 / std::u32::MAX as f32) * 5.0) - 0.5,
        );
        let scale = Mat4::from_scale(scale);
        let r = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
        let y = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
        let p = (rng.next_u32() as f32 / std::u32::MAX as f32) * 1.0 * std::f32::consts::PI;
        let rotate = Mat4::from_euler_angles(r, p, y);
        let transformed_teapot = translate * scale * rotate;
        program.create_object(i.to_string().as_str(), "teapot", transformed_teapot);
    }
    // let monkey = Mesh::parse("./objs/monkey.obj")?;
    // program.new_object("monkey", monkey)?;

    let ambient_light = LightColor::new(1.0, 1.0, 1.0, 0.01);
    let red_light = LightColor::new(1.0, 0.0, 0.0, 1.0);
    let green_light = LightColor::new(0.0, 1.0, 0.0, 1.0);
    let blue_light = LightColor::new(0.0, 0.0, 1.0, 1.0);

    let location_1 = Position::new(0.0, 0.0, -500.0);
    let location_2 = Position::new(0.0, 0.0, 500.0);
    let location_3 = Position::new(0.0, -500.0, 0.0);
    let location_4 = Position::new(0.0, 500.0, 0.0);
    let location_5 = Position::new(-500.0, 0.0, 0.0);
    let location_6 = Position::new(500.0, 0.0, 0.0);

    program.add_light(&location_1, &blue_light)?;
    program.add_light(&location_2, &blue_light)?;
    program.add_light(&location_3, &green_light)?;
    program.add_light(&location_4, &green_light)?;
    program.add_light(&location_5, &red_light)?;
    program.add_light(&location_6, &red_light)?;

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
