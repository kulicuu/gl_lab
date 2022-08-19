#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;
// layout(location=1) in vec3 a_normal;

out highp float r;
out highp float g;
out highp float b;


uniform Stuff {

    mat4 view_mat; // This is a view transform atm.
    // mat4 proj_mat;
};



void main() {
    vec4 camera_space = view_mat * vec4(a_position, 1.0);
    gl_Position = camera_space;

    r = float(3.0 * abs(a_position.x));
    g = float(3.0 * abs(a_position.y));
    b = float(3.0 * abs(a_position.z));
}