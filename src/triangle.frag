#version 150 core

in VS_OUTPUT {
    vec2 texture;
} IN;

out vec4 color;

uniform sampler2D tex;
uniform sampler2D tex2;
uniform float mix_level;

void main() {
    color = mix(texture(tex, IN.texture), texture(tex2, IN.texture), mix_level);
}
