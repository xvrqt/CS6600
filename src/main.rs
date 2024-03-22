// My Libs
use cs6600::{
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
    layout (location = 0) in vec3 vertices;
    layout (location = 1) in vec3 colors;

    uniform mat4 mvp;

    out vec3 clrs;
    void main() {
       gl_Position = mvp * vec4(vertices.x, vertices.y, vertices.z, 1.0);
       clrs = colors;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 460 core
    #define PI 3.141592653

    uniform float time;
    uniform vec2 resolution;

    in vec3 clrs;
    in vec4 gl_FragCoord;
    out vec4 fragColor;

    // Signed Distance to a circle
    float sdCircle(in vec2 o, in vec2 p, in float r) {
        return distance(o,p) - r;
    }

    // My moving circle
    vec2 circle(in float t, in float r) {
        float x = r * 10.0 * sin((2.0*PI)+t); 
        float y = r * 4.0 * cos((2.0*PI*2.0)+t);
        return vec2(x,y);
    }

    // Shifting color palette
    vec3 palette(in float t) {
        vec3 a = vec3(0.5,0.5,0.5);
        vec3 b = vec3(0.5,0.5,0.5);
        vec3 c = vec3(1.0,1.0,1.0);
        vec3 d = vec3(0.268,0.416,0.557);

        return a + b * cos((2.0*PI)*((c*t)+d));
    }

    void main() {
        // Normalized pixel coordinates (from 0 to 1)
        vec2 uv = (gl_FragCoord.xy * 2.0 - resolution.xy) / resolution.y;
        vec2 uv0 = uv; // Save orginal origin

        // How Fast the Circle Moves
        float speed = 0.25;
        float t = time * speed;    

        // Final Output Color
        vec3 finalColor = vec3(0.0);

        float l = sdCircle(circle(t,0.25),uv0,0.5);
        l = 2.0 * sin(l*8. + time) + 2.5;
        int layers = int(round(l));

        for(int i = 0; i < layers; i++) {
            // Circle Center Location
            vec2 circleLoc = circle(t,0.25);
            
            // Repeat in a grid
            uv *= (resolution.x/resolution.y) / 2.0;
            float lf = float(l);
            uv = fract((uv * -1.753) - (lf * 0.1)) - 0.5;
            
            float d = sdCircle(circleLoc,uv,0.5);
            d *= exp(-sdCircle(circleLoc,uv0,0.1));
            // Initial Color
            vec3 col = palette(distance(circle(t,.25),uv0) + t);

            // Rings around the center, move towards center with time
            d = (0.5*(sin(d*2.*PI/4. + time+float(i)))+1.);

            // Vingette around the edges
            float v = length(uv);
            float fade = 1.0 - smoothstep(0.25,0.4,v);
            col = col * pow(d,1.2) * fade;
            finalColor += col / float(layers);
        }

        // Output to screen
        finalColor += clrs * 0.5;
        fragColor = vec4(finalColor, 0.33);
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
