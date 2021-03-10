#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aST;

out vec3 FragPos;
out vec3 vNormal;
out vec2 vST;

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;


uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;

uniform mat4 model_rot;
uniform mat4 view_rot;


void main() {
    gl_Position = proj * view * model * vec4(aPosition, 1.0);
    FragPos = vec3(model * vec4(aPosition, 1.0));
    vNormal = aNormal;
    vec3 dumbNormal = normalize( model_rot * vec4(aNormal, 1.0)).xyz;
    vNormal = dumbNormal; // world space normals get interpolated correctly, maybee???
    vST = aST;
}
