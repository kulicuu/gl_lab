#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;

uniform Stuff {
    mat4 view_mat;
};


void main() {


    vec4 viewed_pos = view_mat * vec4(a_position, 1.0);
    // vec4 viewed_pos = vec4(a_position, 1.0) * view_mat;

    gl_Position = viewed_pos;


    // vec4 t1 = vec4(a_position.x - view_mat[0][3], a_position.y - view_mat[0][3], a_position.z - view_mat[0][1], 1.0);


    // gl_Position = vec4(a_position, 1.0);

    // gl_Position = t1;
}