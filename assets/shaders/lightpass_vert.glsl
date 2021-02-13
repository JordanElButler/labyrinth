#version 330 core

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aST;

out vec2 vST;

void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    vST = aST;
}