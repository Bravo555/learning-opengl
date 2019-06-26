#version 150 core

in vec2 position;
in vec3 color;
in vec2 texture;

out VS_OUTPUT {
    vec3 color;
    vec2 texture;
} OUT;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    OUT.color = color;
    OUT.texture = texture;
}
