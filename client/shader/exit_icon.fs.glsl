#version 330

#ifdef GL_ES
precision mediump float;
#endif

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

float line_width = 0.1;
float margin_x = 0.134;

float in_frame_outline(vec2 st) {
    float acc = 0.;
    vec2 margin = vec2(margin_x, 0.);
    float max_x = 0.6;

    acc += step(margin.x, st.x);
    acc *= step(margin.y, st.y);
    acc *= step(margin.y, 1.000 - st.y);
    if (margin.y + line_width < st.y && st.y < 1. - margin.y - line_width) {
        acc *= step((1. - margin.x - line_width), 1. - st.x);
    }
    acc *= step(1. - max_x, 1. - st.x);

    return acc;
}

float in_arrow_body(vec2 st) {
    float acc = 0.;

    float start = 0.35;
    float stop = 1. - line_width - margin_x;
    float x_in = start < st.x && st.x < stop ? 1. : 0.;
    float y_in = .5 - line_width / 2. < st.y && st.y < .5 + line_width / 2. ? 1. : 0.;
    float in_line = x_in * y_in;

    acc += in_line;
    return acc;
}

float in_arrow_head(in vec2 st) {
    float acc = 0.;
    vec2 point = vec2(1. - line_width / 2. - margin_x, .5);
    st -= point;

    float upper_bound = step(.0, -abs(st.y) - (st.x - line_width / 2.));
    float lower_bound = step(.0, -abs(st.y) - (st.x - line_width / 2. + line_width * sqrt(2.)));
    acc += upper_bound - lower_bound;

    float max_extent = .22;
    acc *= 1. - step(max_extent, abs(st.y));

    return acc;
}

void main() {
    vec2 st = fragTexCoord.xy/u_resolution.xy;
    st.x *= u_resolution.x/u_resolution.y;

    vec3 color_bg = vec3(0.);
    float in_shape = in_frame_outline(st) + in_arrow_body(st) + in_arrow_head(st);
    vec3 color_fg = vec3(in_shape);
    vec3 color = color_bg + color_fg;

    finalColor = vec4(color, 1.0);
}
