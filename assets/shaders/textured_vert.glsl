#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aST;

out vec3 vNormal;
out vec2 vST;

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;

uniform sampler2D texture1;

uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;


void main() {
    gl_Position = proj * view * model * vec4(aPosition, 1.0);
    vNormal = aNormal;
    vST = aST;
}