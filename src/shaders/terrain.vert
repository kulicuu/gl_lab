#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

uniform Stuff {

    mat4 view_mat; // This is a view transform atm.
    mat4 proj_mat;
};

out highp vec3 diffuse_light;

void main() {


    gl_Position = vec4(a_position, 1.0);


}