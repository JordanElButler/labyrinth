#version 330 core

layout (location = 0) out vec3 Position;
layout (location = 1) out vec4 Normal;
layout (location = 2) out vec4 Albedo;
layout (location = 3) out float Metallic;
layout (location = 4) out float Roughness;
layout (location = 5) out float AO;

in vec3 FragPos;
in vec3 vNormal;
in vec2 vST;

uniform mat4 model_rot;
uniform mat4 view_rot;

uniform vec3 albedo;
uniform float metallic;
uniform float roughness;
uniform float ao;

void main() {

    Position = FragPos;
    Normal = vec4(normalize(vNormal), 1.0);
    Albedo = vec4(albedo, 1.0);
    Metallic = metallic;
    Roughness = roughness;
    AO = ao;
}