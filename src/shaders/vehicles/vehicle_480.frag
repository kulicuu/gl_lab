#version 300 es
precision mediump float;

in highp vec3 diffuse_light;

out vec4 FragColor;

void main() {




    FragColor = vec4(0.11 * diffuse_light.x, 0.11 * diffuse_light.y, 0.11 * diffuse_light.z, 0.6);
}
