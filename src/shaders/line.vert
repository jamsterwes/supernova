#version 440 core

uniform vec2 from;
uniform vec2 to;
uniform float width;
uniform vec2 resolution;

layout (location = 0) in vec3 aPos;

void main() {
    // Treat coords as pixels
    vec2 pos = aPos.xy;

    // Set "line width" (height in pixels)
    pos.y *= width;

    // Set line length (in pixels)
    pos.x *= distance(from, to);

    // Rotate line
    float a = atan(to.y - from.y, to.x - from.x);
    float s = sin(a);
	float c = cos(a);
	mat2 m = mat2(c, s, -s, c);
    pos = m * pos;

    // Transpose (in pixels)
    pos += from;

    // Convert back to NDC
    pos /= resolution;
    pos *= 2;
    pos -= vec2(1,1);

    // Return final NDC coordinates
    gl_Position = vec4(pos.x, -pos.y, 0.0, 1.0);
}