#version 440 core

uniform vec4 color;

// For computing circle
uniform vec2 position;
uniform float radius;

out vec4 FragColor;

void main() {

    // Calculate clipping
    float d = length(gl_FragCoord.xy - position - vec2(0.5, 0.5));
    if (d > radius) discard;

    FragColor = color;
}