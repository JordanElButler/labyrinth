#version 330 core

in vec2 vST;

out vec4 Color;

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;

uniform sampler2D my_texture;

void main() {
    vec3 col = texture(my_texture, vST).xyz;
    Color = vec4(col, 1.0);
}