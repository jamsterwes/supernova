#version 440 core

uniform vec2 position;
uniform float radius;
uniform vec2 resolution;

layout (location = 0) in vec3 aPos;

void main() {
    // Treat coords as pixels
    vec2 pos = aPos.xy;

    // Multiply by radius
    pos *= radius;

    // Transpose in pixels
    pos += position;

    // Convert back to NDC
    pos /= resolution;
    pos *= 2;
    pos -= vec2(1,1);

    // Return final NDC coordinates
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}