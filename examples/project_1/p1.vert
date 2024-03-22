#version 460 core
layout (location = 0) in vec3 vertices;
layout (location = 1) in vec3 colors;

// Model-View Perspective Transformation Matrix
uniform mat4 mvp;

// Varying vertex colors
out vec3 clrs;

void main() {
   clrs = colors;
   gl_Position = mvp * vec4(vertices.x, vertices.y, vertices.z, 1.0);
}
