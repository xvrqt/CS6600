pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 460 core

    layout (location = 0) in vec3 vertices;
    layout (location = 1) in vec3 normals;
    layout (location = 2) in mat4 object_transforms;

    uniform mat4 mvp;

    out vec4 mv_point;
    out vec3 mv_normal;

    void main() {
        gl_Position = mvp * object_transforms * vec4(vertices, 1.0);
        // Model - View only transforms for shading
        mv_point = object_transforms * vec4(vertices, 1.0);
        mv_normal = transpose(inverse(mat3(object_transforms))) * normals;
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
            vec4 light_pos = lights[i].position;
            float light_distance =  length(light_pos - mv_point);
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
            float light_attenuation = 1.0 / (pow(light_distance,0.5) + 1.0);
            final_color += light_color * light_attenuation * (diffuse + specular);
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
            vec4 new_light_pos = lights[i].position;
            new_light_pos = new_light_pos - mv_point;
            new_light_pos = normalize(new_light_pos);
            vec3 light_direction = vec3(new_light_pos);

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
            final_color += light_color * (diffuse + specular);
        }

        // Ambient Light
        vec4 ambient = kd * ambient_light_color * ambient_light_color.w;

        fragColor = final_color + ambient;
    }
"#;
