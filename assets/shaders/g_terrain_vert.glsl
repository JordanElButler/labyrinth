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

uniform int numX;
uniform int numZ;


void main() {

    float offsetX = (gl_InstanceID % numX);
    float offsetZ = floor(float(gl_InstanceID) / float(numX));
    
    vec3 newPosition = aPosition +  vec3(offsetX, 0.0, offsetZ);

    gl_Position = proj * view * model * vec4(newPosition, 1.0);
    FragPos = vec3(model * vec4(newPosition, 1.0));
    vec3 dumbNormal = normalize(view_rot * model_rot * vec4(aNormal, 1.0)).xyz;
    vNormal = dumbNormal; 
    vST = aST;
}
