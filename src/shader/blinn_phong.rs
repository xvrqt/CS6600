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

    struct Light {
        vec4 color;
        vec4 position;
    };

    layout (std140, binding = 0) uniform Lights {
        Light lights [100];
    };

    uniform mat4 mv;
    uniform uint num_lights;

    in vec4 mv_point;
    in vec3 mv_normal;

    out vec4 fragColor;

    void main() {
        vec4 final_color = vec4(0,0,0,1);
        for ( uint i = 0; i < num_lights; i++) {
        vec4 light_color = lights[i].color;
        vec4 ambient_light_color = vec4(0.2, 0.2, 0.2, 1.0);
        vec3 light_direction = normalize(-vec3(mv * lights[i].position));

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

        vec3 half_angle = normalize(light_direction + view_direction);

        float cos_phi = dot(reflection, view_direction);
        cos_phi = dot(half_angle, mv_normal);
        cos_phi = max(cos_phi, 0.0);
        vec4 ks = vec4(1.0, 1.0, 1.0, 1.0);
        vec4 specular = ks * pow(cos_phi, 1000);

        // Ambient Light
        vec4 ambient = vec4(0.0, 0,1,0) * ambient_light_color * 2.0;

        // Output to screen
        final_color += light_color * (diffuse + specular) + ambient;
        }
        fragColor = final_color;
    }
"#;
