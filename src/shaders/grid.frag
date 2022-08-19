#version 300 es
precision mediump float;

out vec4 FragColor;

in highp float r;
in highp float g;
in highp float b;

void main() {

    

    FragColor = vec4(r, g, b, 1.0);


    // FragColor = vec4(0.63, 0.7, 0.81, 1.0);
}
