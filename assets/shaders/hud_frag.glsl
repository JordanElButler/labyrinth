#version 330 core

in vec2 vST;

out vec4 Color;

uniform vec2 iResolution;

uniform sampler2D my_texture;


float radius = 2.5;
vec2 center = iResolution.xy / 2.0;
    
void main() {
    if (distance(center, gl_FragCoord.xy ) < radius) {
        Color = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        discard;
    }
}