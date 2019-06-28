#version 150 core

in vec3 position;
in vec3 color;
in vec2 texture;

out VS_OUTPUT {
    vec3 color;
    vec2 texture;
} OUT;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    OUT.color = color;
    OUT.texture = texture;
}
