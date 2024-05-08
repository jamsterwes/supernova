#version 440 core

uniform sampler2D tex;
uniform vec2 resolution;

in vec2 uv;
out vec4 FragColor;

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec4 field = texture(tex, uv);
    float V = length(field.xy);
    float H = (atan(field.y, field.x) + (3.1415 / 2)) / 3.1415;
    // FragColor = vec4(hsv2rgb(vec3(H, 1, V)), 1);
    
    FragColor = vec4(V,V,V, 1);
}