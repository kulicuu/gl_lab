#version 300 es
precision mediump float;
layout(std140) uniform;

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

uniform Stuff {
    mat4 model_mat; // not actually such atm.  This is almost a vehicle inertial frame vector controller
    mat4 view_mat; // This is a view transform atm.
    // mat4 model_mat; // Model to world space transform. 
    // mat4 proj_mat;
};

out highp vec3 diffuse_light;

void main() {

    vec4 world_space_pos = model_mat * vec4(a_position, 1.0);

    vec4 camera_space_pos = view_mat * world_space_pos;

    // vec4 model_pos = view_mat * vec4(a_position, 1.0);
    // vec4 world_pos = model_mat * model_pos;
    // vec4 camera_pos = next_mat * world_pos;



    // vec4 camera = next_mat * world_pos;

    // vec4 model_norm = view_mat * vec4(a_normal, 1.0); // maybe the homogeneous coordinate
    // vec4 world_norm = model_mat * model_norm;
    // vec4 camera_norm = next_mat * world_norm;

    gl_Position = camera_space_pos;

    vec4 light_position = vec4(0.5, 0.5, 0.5, 1.0);
    vec3 light_color = vec3(0.8, 0.05, 0.11);
    float light_intensity = 2.1;

    highp vec3 light_color_intensity = light_color * light_intensity;
    highp float distance_from_light = distance(camera_space_pos, light_position);
    highp vec3 normal_view_space = normalize(view_mat * light_position).xyz;
    highp vec4 light_position_view_space = view_mat * light_position;
    highp vec3 light_direction_view_space = normalize((light_position_view_space - camera_space_pos).xyz);

    highp float diffuse_strength = clamp(dot(normal_view_space, light_direction_view_space), 0.0, 1.0);
    diffuse_light = (light_color_intensity * diffuse_strength) / (distance_from_light * distance_from_light);
}