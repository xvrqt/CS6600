use super::{Fragment, Shader, Vertex};

// Aliases so we can keep track of what types of shaders we're using
pub struct BlinnPhongVertexShader<'a>(pub Shader<'a, Vertex>);
pub struct BlinnPhongFragmentShader<'a>(pub Shader<'a, Fragment>);

// Convenience for constructors that operate on more agnostice types
impl<'a> From<Shader<'a, Vertex>> for BlinnPhongVertexShader<'a> {
    fn from(s: Shader<'a, Vertex>) -> BlinnPhongVertexShader<'a> {
        BlinnPhongVertexShader(s)
    }
}

impl<'a> From<Shader<'a, Fragment>> for BlinnPhongFragmentShader<'a> {
    fn from(s: Shader<'a, Fragment>) -> BlinnPhongFragmentShader<'a> {
        BlinnPhongFragmentShader(s)
    }
}

// Unwrap for when we don't want type checking (like in constructors)
impl<'a> std::ops::Deref for BlinnPhongVertexShader<'a> {
    type Target = Shader<'a, Vertex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::Deref for BlinnPhongFragmentShader<'a> {
    type Target = Shader<'a, Fragment>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 460 core

    layout (location = 0) in vec4 vertices;
    layout (location = 1) in vec3 normals;

    uniform mat4 mv;
    uniform mat4 mvp;
    uniform mat3 mvn;

    out vec4 mv_point;
    out vec3 mv_normal;

    void main() {
       gl_Position = mvp * vertices;

       // Model - View only transforms for shading
       mv_point = mv * vertices; 
       mv_normal = mvn * normals;
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 460 core

    // Dynamic lighting
    struct Light {
        vec4 color;
        vec4 position;
    };
    layout (std140, binding = 0) uniform Lights {
        Light lights [100];
    };
    uniform mat4 mv; // Model-View Matrix transforms the light source positions
    uniform uint num_lights;

    // Ambient Lighting
    uniform vec4 ambient_light_color;
    uniform float ambient_intensity;

    // Model-View Position and Normals for shading calculations
    in vec4 mv_point;
    in vec3 mv_normal;

    // Color of this fragment
    out vec4 fragColor;

    void main() {
        // Material Properties
        vec4 kd = vec4(0.9, 0.9, 0.9, 1.0);

        // Loop accumulates color from lights sources in this vec
        vec4 final_color = vec4(0,0,0,1);
        for(uint i = 0; i < num_lights; i++) {
            vec4 light_color = lights[i].color;
            // Why don't we have to transform the light location with 'mv' ?
            vec4 new_light_pos = mv * lights[i].position;
            new_light_pos = new_light_pos - mv_point;
            new_light_pos = normalize(-new_light_pos);
            vec3 light_direction = vec3(new_light_pos);

            // Geometry Term
            float cos_theta = dot(mv_normal, vec3(light_direction));
            float geometry_term = max(cos_theta, 0.0);

            // Diffuse Term
            vec4 diffuse = kd * geometry_term;

            // Specular Term
            vec3 reflection = 2.0 * dot(mv_normal, light_direction) * mv_normal - light_direction;
            reflection = normalize(reflection);
            vec3 view_direction = normalize(vec3(-mv_point));

            vec3 half_angle = normalize(light_direction + view_direction);

            float cos_phi = dot(reflection, view_direction);
            cos_phi = dot(half_angle, mv_normal);
            cos_phi = max(cos_phi, 0.0);
            vec4 ks = vec4(1.0, 1.0, 1.0, 1.0);
            vec4 specular = ks * pow(cos_phi, 1000);


            // Output to screen
            final_color += light_color * (diffuse + specular);
        }

        // Ambient Light
        vec4 ambient = kd * ambient_light_color * ambient_intensity;

        fragColor = final_color + ambient;
    }
"#;
