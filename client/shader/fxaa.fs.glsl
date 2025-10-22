#version 330

#ifdef GL_ES
precision mediump float;
#endif// GL_ES

#define FXAA_REDUCE_MIN (1./128.)
#define FXAA_REDUCE_MUL (1./8.)
#define FXAA_SPAN_MAX 8.

in vec2 fragTexCoord;
in vec4 fragColor;
in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;
uniform sampler2D u_sampler0;
uniform vec4 colDiffuse;

void main()
{
    vec4 color;
    vec2 resolution = vec2(1600., 900.);
    vec2 inverse_resolution = vec2(1./resolution.x, 1./resolution.y);

    vec3 rgb_tl = texture(u_sampler0, fragTexCoord.xy + (vec2(-1., -1.)) * inverse_resolution).xyz;
    vec3 rgb_tr = texture(u_sampler0, fragTexCoord.xy + (vec2(1., -1.)) * inverse_resolution).xyz;
    vec3 rgb_bl = texture(u_sampler0, fragTexCoord.xy + (vec2(-1., 1.)) * inverse_resolution).xyz;
    vec3 rgb_br = texture(u_sampler0, fragTexCoord.xy + (vec2(1., 1.)) * inverse_resolution).xyz;
    vec3 rgb_m  = texture(u_sampler0, fragTexCoord.xy).xyz;

    vec3 luma = vec3(0.299, 0.587, 0.114);

    float luma_tl = dot(rgb_tl, luma);
    float luma_tr = dot(rgb_tr, luma);
    float luma_bl = dot(rgb_bl, luma);
    float luma_br = dot(rgb_br, luma);
    float luma_m  = dot(rgb_m, luma);
    float luma_min = min(luma_m, min(min(luma_tl, luma_tr), min(luma_bl, luma_br)));
    float luma_max = max(luma_m, max(max(luma_tl, luma_tr), max(luma_bl, luma_br)));

    vec2 luma_direction;
    luma_direction.x = -((luma_tl + luma_tr) - (luma_bl + luma_br));
    luma_direction.y =  ((luma_tl + luma_bl) - (luma_tr + luma_br));

    float direction_reduce = max((luma_tl + luma_tr + luma_bl + luma_br) * (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);
    float direction_scale = 1./(min(abs(luma_direction.x), abs(luma_direction.y)) + direction_reduce);

    luma_direction = clamp(luma_direction * direction_scale, -FXAA_SPAN_MAX, FXAA_SPAN_MAX) * inverse_resolution;

    vec3 sample_a1 = texture(u_sampler0, fragTexCoord + luma_direction * (1.0/3.0 - 0.5)).xyz;
    vec3 sample_a2 = texture(u_sampler0, fragTexCoord + luma_direction * (2.0/3.0 - 0.5)).xyz;
    vec3 color_a = (sample_a1 + sample_a2) * 0.5;

    vec3 sample_b1 = texture(u_sampler0, fragTexCoord + luma_direction * -0.5).xyz;
    vec3 sample_b2 = texture(u_sampler0, fragTexCoord + luma_direction * 0.5).xyz;
    vec3 color_b = color_a * 0.5 + (sample_b1 + sample_b2) * 0.25;

    float luma_b = dot(color_b, luma);
    if ((luma_b < luma_min) || (luma_b > luma_max)) {
        color = vec4(color_a, 1.);
    } else {
        color = vec4(color_b, 1.);
    }

    finalColor = color * colDiffuse;
}
