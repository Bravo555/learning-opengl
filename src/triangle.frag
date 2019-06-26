#version 150 core

in VS_OUTPUT {
    vec3 color;
    vec2 texture;
} IN;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, IN.texture);
}
