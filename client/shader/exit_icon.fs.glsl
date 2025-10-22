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

float in_box(in vec2 st, in float upper_bound) {
    float accumulator = 1.;
    float edge = 1. - upper_bound;
    float top = step(edge, 1. - st.y);
    float right = step(edge, 1. - st.x);

    accumulator *= top * right;
    return accumulator;
}

float in_frame_outline(in vec2 st) {
    float acc = 0.;
    vec2 margin = vec2(.2, .1);

    acc += step(margin.x, st.x);
    acc *= step(margin.y, st.y);
    acc *= step(margin.y, 1. - st.y);
    if (margin.y + line_width < st.y && st.y < 1. - margin.y - line_width) {
        acc *= step((1. - margin.x - line_width), 1. - st.x);
    }
    acc *= step(0.480, 1. - st.x);

    return acc;
}

float in_arrow_body(in vec2 st) {
    float acc = 0.;

    float start = .4;
    float stop = .8;
    float x_in = start < st.x && st.x < stop ? 1. : 0.;
    float y_in = .5 - line_width / 2. < st.y && st.y < .5 + line_width / 2. ? 1. : 0.;
    float in_line = x_in * y_in;

    acc += in_line;
    return acc;
}

float in_arrow_head(in vec2 st) {
    float acc = 0.;
    st = vec2(st.x - .8, st.y - .5);

    float upper_bound = step(.0, -abs(st.y) - (st.x - line_width / 2.));
    float lower_bound = step(.0, -abs(st.y) - (st.x - line_width / 2. + line_width * sqrt(2.)));
    acc += upper_bound - lower_bound;

    float max_extent = .2;
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
