#version 330 core

in vec3 FragPos;
in vec3 vNormal;
in vec2 vST;

out vec4 Color;

uniform vec2 iResolution;
uniform vec4 iMouse;
uniform float iTimeDelta;

uniform vec3 light_pos;
uniform vec3 light_col;
uniform vec3 view_pos;

uniform sampler2D texture1;

void main() {
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * light_col;
    vec3 norm = normalize(vNormal);
    vec3 light_dir = normalize(light_pos - FragPos);
    vec4 object_color = texture(texture1, vST);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_col;

    float specularStrength = 0.5;
    vec3 viewDir = normalize(view_pos - FragPos);
    vec3 reflectDir = reflect(-light_dir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * light_col;

    vec4 result = vec4((ambient + diffuse + specular), 1.0) * object_color;
    Color = result;


}