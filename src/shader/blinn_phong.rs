pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 460 core

    // Vertices in model space
    layout (location = 0) in vec3 vertices;
    // Normal pseudo-vectors in model space
    layout (location = 1) in vec3 normals;
    // Per object model-world transforms
    layout (location = 2) in mat4 object_mw_transforms;
    // location = 3,4,5 reserved by `object_mw_transforms`
    // Per object model-world normal pseudo-vector transform
    layout (location = 6) in mat3 object_mw_normal_transforms;
    // location = 7,8 reserved by `object_mw_normal_transforms`

    // View-Projection transformation matrix
    uniform mat4 mvp;
    uniform mat4 mv;
    uniform mat3 mvn;

    out vec4 mv_point;
    out vec3 mv_normal;

    void main() {
        gl_Position = mvp * object_mw_transforms * vec4(vertices, 1.0);
        // Model - View only transforms for shading
        mv_point = mv * object_mw_transforms * vec4(vertices, 1.0);
        mv_normal = mvn * object_mw_normal_transforms * normals;
    }
"#;

pub const BLINN_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 460 core

    // Dynamic lighting
    struct Light {
        vec4 color;
        vec4 position;
    };
    layout (std140, binding = 1) uniform Lights {
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
            vec4 light_color = vec4(vec3(lights[i].color), 1.0) * lights[i].color.w;
            vec4 light_pos = mv * lights[i].position;
            vec3 light_direction = vec3(normalize(light_pos - mv_point));

            // Geometry Term
            float cos_theta = dot(mv_normal, vec3(light_direction));
            float geometry_term = max(cos_theta, 0.0);

            // Diffuse Term
            vec4 diffuse = kd * geometry_term;

            // Specular Term
            vec3 reflection = 2.0 * dot(mv_normal, light_direction) * mv_normal - light_direction;
            reflection = normalize(reflection);
            vec3 view_direction = normalize(vec3(-mv_point));

            float cos_phi = dot(reflection, view_direction);
            cos_phi = max(cos_phi, 0.0);
            vec4 ks = vec4(1.0, 1.0, 1.0, 1.0);
            vec4 specular = ks * pow(cos_phi, 1000);


            // Output to screen
            // Clamp ?
            final_color += light_color * (diffuse + specular);
        }

        // Ambient Light
        vec4 ambient = kd * ambient_light_color * ambient_light_color.w;

        fragColor = final_color + ambient;
    }
"#;

pub const PHONG_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 460 core

    // Dynamic lighting
    struct Light {
        vec4 color;
        vec4 position;
    };
    layout (std140, binding = 1) uniform Lights {
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
            vec4 light_color = vec4(vec3(lights[i].color), 1.0) * lights[i].color.w;
            vec4 light_pos = mv * lights[i].position;
            vec3 light_direction = vec3(normalize(light_pos - mv_point));

            // Geometry Term
            float cos_theta = dot(mv_normal, vec3(light_direction));
            float geometry_term = max(cos_theta, 0.0);

            // Diffuse Term
            vec4 diffuse = kd * geometry_term;

            // Specular Term
            vec3 view_direction = normalize(vec3(-mv_point));
            vec3 half_angle = normalize(light_direction + view_direction);

            float cos_phi = dot(half_angle, mv_normal);
            cos_phi = max(cos_phi, 0.0);
            vec4 ks = vec4(1.0, 1.0, 1.0, 1.0);
            vec4 specular = ks * pow(cos_phi, 1000);


            // Output to screen
            // Clamp ?
            final_color += light_color * (diffuse + specular);
        }

        // Ambient Light
        vec4 ambient = kd * ambient_light_color * ambient_light_color.w;

        fragColor = final_color + ambient;
    }
"#;
