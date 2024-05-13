#version 440 core

uniform vec4 color;

// For computing circle
uniform vec2 position;
uniform float radius;

out vec4 FragColor;

void main() {

    // Calculate clipping
    float d = length(gl_FragCoord.xy - position - vec2(0.5, 0.5));

    // For smoothing (in px)
    const float S = 1.5;
    
    // Set alpha
    float remap = (d - (radius - S)) / S;
    remap = clamp(remap, 0, 1);

    float alpha = smoothstep(1.0, 0.0, remap);

    FragColor = vec4(color.rgb, color.a * alpha);
}