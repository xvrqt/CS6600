// My Libs
use cs6600::{
    load::load_obj,
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

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 460 core
    layout (location = 0) in vec4 vertices;
    layout (location = 1) in vec3 normals;

    uniform mat4 mv;
    uniform mat4 mvp;
    uniform mat3 mvn;

    out vec4 mv_point;
    out vec3 mv_normal;
    out vec3 wut;

    void main() {
       gl_Position = mvp * vertices;
       // Model - View only transforms for shading
       mv_point = mv * vertices; 
       mv_normal = mvn * normals;
       wut = normals;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 460 core

    // uniform float time;
    // uniform vec2 resolution;

    in vec4 mv_point;
    in vec3 mv_normal;
    in vec3 wut;

    out vec4 fragColor;

    void main() {
        vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
        vec4 ambient_light_color = vec4(0.2, 0.2, 0.2, 1.0);
        vec3 light_direction = normalize(vec3(0.0, 0.0, 1.0));

        // Geometry Term
        float cos_theta = dot(mv_normal, light_direction);
        float geometry_term = max(cos_theta, 0.0);

        // Diffuse Term
        vec4 kd = vec4(1.0, 0.0, 0.0, 1.0);
        vec4 diffuse = kd * geometry_term;

        // Specular Term
        vec3 reflection = 2.0 * dot(mv_normal, light_direction) * mv_normal - light_direction;
        reflection = normalize(reflection);
        vec3 view_direction = normalize(vec3(-mv_point));

        float cos_phi = dot(reflection, view_direction);
        cos_phi = max(cos_phi, 0.0);
        vec4 ks = vec4(1.0, 1.0, 1.0, 1.0);
        vec4 specular = ks * pow(cos_phi, 1000);

        // Ambient Light
        vec4 ambient = vec4(0.0, 0,1,0) * ambient_light_color * 2.0;

        // // Normalized pixel coordinates (from 0 to 1)
        // vec2 uv = (gl_FragCoord.xy * 2.0 - resolution.xy) / resolution.y;
        // vec2 uv0 = uv; // Save orginal origin

        // Output to screen
        // vec3 final_color = vec3(0.5, 0.1, 0.9);
        // fragColor = vec4(final_color, 1.0);
        fragColor = light_color * (diffuse + specular) + ambient;
        // fragColor *= 0.01;
        // vec3 w = vec3(0.5,0.5,0.5) + (wut / 2.0);
        // fragColor += vec4(w, 1.0);
    }
"#;

#[allow(non_snake_case)]
fn main() -> Result<(), GLError> {
    // GLFW lib handle, window handle, and event loop for that window handle
    let (mut glfw, mut window, events) = window::create_default()?;

    // Load function pointers from the user's linked OpenGL library
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Compile Shaders
    let vertex_shader = Shader::<Vertex>::new(VERTEX_SHADER_SOURCE)?;
    let fragment_shader = Shader::<Fragment>::new(FRAGMENT_SHADER_SOURCE)?;

    let n = 12;
    let mut obj = load_obj("./src/wires.obj")?;
    // obj.vertices = obj.vertices[0..n].to_vec();
    // obj.normals = obj.normals[0..n].to_vec();
    // obj.indices = obj.indices[0..n].to_vec();
    // obj.uv = Vec::new();
    // println!("{:#?}", obj);

    // Link Shaders to Program
    let program = GLProgram::builder()
        .attach_vertex_shader(vertex_shader)
        .attach_fragment_shader(fragment_shader)
        .link_shaders()?
        // .enable_uniform(MagicUniform::TIME) // Will set the float 'time' as a uniform every call
        // .enable_uniform(MagicUniform::RESOLUTION) // Will pass the 'resolution' as a vec2
        .vao_from_obj("cube", &obj)?;

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

        // println!("GAY");
        // for n in obj.normals.iter() {
        //     let y = n.clone();
        //     let nn = mv.transform_vec3(y);
        //     println!("Normal was: {:?}  -->  {:?}", n, nn);
        // }

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
