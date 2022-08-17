#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

uniform Stuff {
    mat4 view_mat;
};

void main() {
    // a_normal;
    vec4 viewed_pos = view_mat * vec4(a_position, 1.0);
    gl_Position = viewed_pos;
}