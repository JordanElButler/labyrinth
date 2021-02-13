#version 330 core

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;

in vec4 vColor;

out vec4 Color;


void main() {
    if (vColor != vec4(0.0, 0.0, 0.0, 0.0)) {
        Color = vColor;

    } else {
        Color = vec4(0.0, 1.0, 0.0, 1.0);
    }
}