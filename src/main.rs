// TODO: Remove this, we should follow RUST's protocol
#![allow(non_upper_case_globals)]

// My Libs
use cs6600::{
    shader::{Fragment, Vertex},
    types::*,
    window, GLError, GLProgram, Shader,
};

use gl::types::*;
use glfw::{Action, Context, Key};

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::time::SystemTime;

const vertexShaderSource: &str = r#"
    #version 460 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 460 core
    #define PI 3.141592653

    uniform float time;
    uniform vec2 resolution;
    // uniform vec2 clr;

    out vec4 fragColor;
    in vec4 gl_FragCoord;

    float sdCircle(in vec2 o, in vec2 p, in float r) {
        return distance(o,p) - r;
    }

    vec2 circle(in float t, in float r) {
        float x = r * 10.0 * sin((2.0*PI)+t); 
        float y = r * 4.0 * cos((2.0*PI*2.0)+t);
        return vec2(x,y);
    }

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
        float speed = .25;
        float t = time * speed;    
        // Final Color
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
        fragColor = vec4(finalColor, 1.0);
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

    // Link Shaders to Program
    let program = GLProgram::builder()
        .attach_vertex_shader(vertex_shader)
        .attach_fragment_shader(fragment_shader)
        .link_shaders()?;

    // let clr = GL2F(0.5, 0.1);
    // let gay: GL2FV = GL2FV(vec![(0.5, 0.1)]);

    // program.set_uniform("clr", GL2F(0.5, 0.1))?;
    // program.set_uniform("gay", gay)?;

    let VAO = unsafe {
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        // let vertices: [f32; 9] = [-1.0, -1.0, 0.0, 3.0, -1.0, 0.0, -1.0, 3.0, 0.0];
        let vertices: [f32; 9] = [
            -1.0, -1.0, 0.0, // left
            3.0, -1.0, 0.0, // right
            -1.0, 3.0, 0.0, // top
        ];
        let (mut VBO, mut VAO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        VAO
    };

    // Time
    let time = SystemTime::now();
    program.set_uniform("resolution", GL2F(1000.0, 1000.0))?;

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events, &program);

        // Update the time variable
        if let Ok(elapsed) = time.elapsed() {
            program.set_uniform("time", GL1F(elapsed.as_secs_f32() as GLfloat))?;
        }

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::UseProgram(program.id());
            gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

// NOTE: not the same version as in common.rs!
fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    program: &GLProgram,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                program
                    .set_uniform("resolution", GL2F(width as f32, height as f32))
                    .expect("AAA");
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
