// TODO: Remove this, we should follow RUST's protocol
#![allow(non_upper_case_globals)]

// My Libs
use cs6600::{
    shader::{Fragment, Vertex},
    types::*,
    uniform::MagicUniform,
    vao::Vertex3,
    window,
    window::FrameEvents,
    GLError, GLProgram, Shader,
};

use gl::types::*;
use glfw::{Action, Context, Key};

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const vertexShaderSource: &str = r#"
    #version 460 core
    layout (location = 0) in vec3 vertices;
    void main() {
       gl_Position = vec4(vertices.x, vertices.y, vertices.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 460 core
    #define PI 3.141592653

    // uniform float time;
    // uniform vec2 resolution;

    in vec4 gl_FragCoord;
    out vec4 fragColor;

    // // Signed Distance to a circle
    // float sdCircle(in vec2 o, in vec2 p, in float r) {
    //     return distance(o,p) - r;
    // }
    //
    // // My moving circle
    // vec2 circle(in float t, in float r) {
    //     float x = r * 10.0 * sin((2.0*PI)+t); 
    //     float y = r * 4.0 * cos((2.0*PI*2.0)+t);
    //     return vec2(x,y);
    // }
    //
    // // Shifting color palette
    // vec3 palette(in float t) {
    //     vec3 a = vec3(0.5,0.5,0.5);
    //     vec3 b = vec3(0.5,0.5,0.5);
    //     vec3 c = vec3(1.0,1.0,1.0);
    //     vec3 d = vec3(0.268,0.416,0.557);
    //
    //     return a + b * cos((2.0*PI)*((c*t)+d));
    // }
    //
    // void main() {
    //     // Normalized pixel coordinates (from 0 to 1)
    //     vec2 uv = (gl_FragCoord.xy * 2.0 - resolution.xy) / resolution.y;
    //     vec2 uv0 = uv; // Save orginal origin
    //
    //     // How Fast the Circle Moves
    //     float speed = 0.25;
    //     float t = time * speed;    
    //
    //     // Final Output Color
    //     vec3 finalColor = vec3(0.0);
    //
    //     float l = sdCircle(circle(t,0.25),uv0,0.5);
    //     l = 2.0 * sin(l*8. + time) + 2.5;
    //     int layers = int(round(l));
    //
    //     for(int i = 0; i < layers; i++) {
    //         // Circle Center Location
    //         vec2 circleLoc = circle(t,0.25);
    //         
    //         // Repeat in a grid
    //         uv *= (resolution.x/resolution.y) / 2.0;
    //         float lf = float(l);
    //         uv = fract((uv * -1.753) - (lf * 0.1)) - 0.5;
    //         
    //         float d = sdCircle(circleLoc,uv,0.5);
    //         d *= exp(-sdCircle(circleLoc,uv0,0.1));
    //         // Initial Color
    //         vec3 col = palette(distance(circle(t,.25),uv0) + t);
    //
    //         // Rings around the center, move towards center with time
    //         d = (0.5*(sin(d*2.*PI/4. + time+float(i)))+1.);
    //
    //         // Vingette around the edges
    //         float v = length(uv);
    //         float fade = 1.0 - smoothstep(0.25,0.4,v);
    //         col = col * pow(d,1.2) * fade;
    //         finalColor += col / float(layers);
    //     }

        // Output to screen
        // fragColor = vec4(finalColor, 1.0);
        void main() {
        fragColor = vec4(0.5, 0.1, 0.9, 1.0);
    }
"#;

#[allow(non_snake_case)]
fn main() -> Result<(), GLError> {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // GLFW lib handle, window handle, and event loop for that window handle
    let (mut glfw, mut window, events) = window::create_default()?;

    // Load function pointers from the user's linked OpenGL library
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Compile Shaders
    let vertex_shader = Shader::<Vertex>::new(vertexShaderSource)?;
    let fragment_shader = Shader::<Fragment>::new(fragmentShaderSource)?;

    // Generate Object Data
    let triangle = Vertex3(vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0]);

    // Link Shaders to Program
    let mut program = GLProgram::builder()
        .attach_vertex_shader(vertex_shader)
        .attach_fragment_shader(fragment_shader)
        .link_shaders()?;
    // .enable_uniform(MagicUniform::TIME) // Will set the float 'time' as a uniform every call
    // .enable_uniform(MagicUniform::RESOLUTION); // Will pass the 'resolution' as a vec2

    program.vao("triangle").attribute("vertices", triangle)?;

    let render_queue = vec![program];

    // render loop
    let mut frame_events = FrameEvents {
        // time: if let Ok(elapsed) = time.elapsed() { elapsed.as_secs_f32() } else { 0.0 },
        time: glfw.get_time() as f32,
        resolution: match window.get_size() {
            (a, b) => (a as f32, b as f32),
        },
    };

    while !window.should_close() {
        // Process events, and extract relevant program details
        process_events(&glfw, &mut window, &events, &mut frame_events)?;

        // RENDER
        // Flags that are used to set 'magic' uniforms such as 'time' or 'mouse position'or each program, render each VAO
        for program in render_queue.iter() {
            program.draw(&frame_events)?;
        }

        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

fn process_events(
    glfw: &glfw::Glfw,
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    frame_events: &mut FrameEvents,
) -> Result<(), GLError> {
    frame_events.time = glfw.get_time() as f32;
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // Update Viewport, and Resolution Shader Uniform
            glfw::WindowEvent::FramebufferSize(width, height) => {
                frame_events.resolution = (width as f32, height as f32);
                // program.set_uniform("resolution", GL2F(width as GLfloat, height as GLfloat))?;
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
    Ok(())
}
