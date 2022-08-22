#version 300 es
precision mediump float;

// in highp vec3 diffuse_light;

flat in int instance_id;

out vec4 FragColor;

void main() {

    
    // Nice effect
    // FragColor = vec4(0.0087 * float(instance_id), 0.0, 0.0, 0.5);

    // FragColor = vec4(float((instance_id % 4) / 4), 0.0, float(((instance_id + 1) % 4) / 4), 0.5);


    float x1 = float(instance_id % 4);
    float x2 = float((instance_id + 1) % 4);

    FragColor = vec4(
        (x1 / 3.0) * 0.7,
        0.0,
        (x2 / 3.0) * 0.7,
        0.8
    );
}
