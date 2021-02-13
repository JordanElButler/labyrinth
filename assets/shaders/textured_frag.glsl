#version 330 core

in vec3 vNormal;
in vec2 vST;

out vec4 Color;

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;

uniform sampler2D texture1;

void main() {
    Color = texture(texture1, vST);
}