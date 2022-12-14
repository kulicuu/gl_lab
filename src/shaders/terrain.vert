#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;
// layout(location=1) in vec3 a_normal;

uniform Stuff {

    mat4 view_mat; // This is a view transform atm.
    // mat4 proj_mat;
};

// out highp vec3 diffuse_light;

flat out int instance_id;

void main() {

    // instance_id = gl_InstanceID;
    instance_id = gl_VertexID;
    // gl_VertexID;
    // a_position is already in world space the way it's set up

    // instance_id = gl_InstanceId;

    vec4 camera_space = view_mat * vec4(a_position, 1.0);

    gl_Position = camera_space;


}