#version 330 core
/*
modified from
https://64.github.io/tonemapping/#uncharted-2
*/
in vec2 vST;

out vec4 Color;

uniform sampler2D my_texture;

uniform vec3 W;
uniform float exposure_bias;

vec3 uncharted2_tonemap_partial(vec3 x)
{
    float A = 0.15f;
    float B = 0.50f;
    float C = 0.10f;
    float D = 0.20f;
    float E = 0.02f;
    float F = 0.30f;
    return ((x*(A*x+C*B)+D*E)/(x*(A*x+B)+D*F))-E/F;
}

vec3 uncharted2_filmic(vec3 v)
{
    //float exposure_bias = 2.0f;
    vec3 curr = uncharted2_tonemap_partial(v * exposure_bias);

    //vec3 W = vec3(11.2f);
    vec3 white_scale = vec3(1.0f) / uncharted2_tonemap_partial(W);
    return curr * white_scale;
}

void main() {
    vec3 col = texture(my_texture, vST).xyz;
    Color = vec4(uncharted2_filmic(col), 1.0);
}