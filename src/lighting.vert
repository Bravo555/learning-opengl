#version 330 core

in vec3 position;
in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 fragNormal;
out vec3 fragPos;

void main() {
    fragPos = vec3(model * vec4(position, 1.0));
    fragNormal = mat3(transpose(inverse(model))) * normal;
    gl_Position = projection * view * model * vec4(position, 1.0);
}
