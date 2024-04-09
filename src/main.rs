// My Libs
use cs6600::{GLError, GLProgram, LightColor, Mesh, Position};

fn main() -> Result<(), GLError> {
    let mut program = GLProgram::phong()?;

    let teapot = Mesh::parse("./objs/teapot.obj")?;
    program.new_object("teapot", teapot)?;
    let monkey = Mesh::parse("./objs/monkey.obj")?;
    program.new_object("monkey", monkey)?;

    let ambient_light = LightColor::new(1.0, 1.0, 1.0, 0.1);
    let location_1 = Position::new(-10.0, -5.0, -5.0);
    let location_2 = Position::new(10.0, -5.0, 5.0);
    let location_3 = Position::new(0.0, 10.0, 0.0);

    program.add_light(&location_1, &LightColor::RED)?;
    program.add_light(&location_2, &LightColor::BLUE)?;
    program.add_light(&location_3, &LightColor::GREEN)?;
    program.ambient_light(&ambient_light)?;

    while program.draw().is_ok() {}
    Ok(())
}
