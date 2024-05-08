#version 440 core

uniform sampler2D tex;
uniform vec2 resolution;

in vec2 uv;
out vec4 FragColor;

// left, right, top, bottom
vec4 ReadEdges(vec2 coords) {
    vec4 velC = texture(tex, coords);
    vec4 velL = texture(tex, coords - vec2(1,0) / resolution);
    vec4 velT = texture(tex, coords + vec2(0,1) / resolution);
    return vec4(velL.x, velC.x, velT.y, velC.y);
}

vec2 EdgesToVelocity(vec4 edges) {
    return vec2(edges.r + edges.g, edges.b + edges.a);
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec4 edges = ReadEdges(uv);
    vec2 velocity = EdgesToVelocity(edges);
    float V = length(velocity);
    float H = (atan(velocity.y, velocity.x) + 3.14/2) / 3.14;
    FragColor = vec4(hsv2rgb(vec3(H, 1, V)), 1);
}