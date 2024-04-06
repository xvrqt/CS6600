pub const VERTEX_SHADER_SOURCE: &str = r#"
#version 460 core
layout (location = 0) in vec3 vertices;

void main() {
   gl_Position = vec4(vertices.x, vertices.y, vertices.z, 1.0);
}
"#;
