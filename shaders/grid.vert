#version 440 core

layout (location = 0) in vec3 aPos;

out vec2 uv;

void main() {
    uv = (aPos.xy + vec2(1,1)) / vec2(2,2);
    uv.y = 1 - uv.y;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}