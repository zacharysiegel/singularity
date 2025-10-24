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

uniform sampler2D texture0;
uniform vec4 colDiffuse;

uniform float u_button_size;
uniform vec2 u_button_origin;

float line_width = 0.135;
float margin_x = 0.;

bool in_frame_outline(vec2 st) {
    float acc = 0.;
    vec2 margin = vec2(margin_x, 0.);
    float max_x = 0.75;

    acc += step(margin.x, st.x);
    acc *= step(margin.y, st.y);
    acc *= step(margin.y, 1.000 - st.y);
    if (margin.y + line_width < st.y && st.y < 1. - margin.y - line_width) {
        acc *= step((1. - margin.x - line_width), 1. - st.x);
    }
    acc *= step(1. - max_x, 1. - st.x);

    return acc == 1.;
}

bool in_arrow_body(vec2 st) {
    bool acc = false;

    float start = 0.35;
    float stop = 1. - line_width - margin_x;
    bool x_in = start < st.x && st.x < stop ? true : false;
    bool y_in = .5 - line_width / 2. < st.y && st.y < .5 + line_width / 2. ? true : false;
    bool in_line = x_in && y_in;

    acc = acc || in_line;
    return acc;
}

bool in_arrow_head(in vec2 st) {
    bool acc = false;
    vec2 point = vec2(1. - line_width / 2. - margin_x, .5);
    st -= point;

    float upper_bound = step(.0, -abs(st.y) - (st.x - line_width / 2.));
    float lower_bound = step(.0, -abs(st.y) - (st.x - line_width / 2. + line_width * sqrt(2.)));
    acc = acc || (upper_bound - lower_bound > 0.);

    float max_extent = .26;
    acc = acc && (1. - step(max_extent, abs(st.y)) > 0.);

    return acc;
}

vec2 st(vec2 givenCoord, vec2 resolution, vec2 target_origin, vec2 target_dimensions) {
    vec2 st = givenCoord * resolution;
    st.y = resolution.y - st.y;
    st = (st - target_origin) / target_dimensions;
    return st;
}

void main() {
    vec2 st = st(fragTexCoord, u_resolution, u_button_origin, vec2(u_button_size));

    vec4 color_bg = texture(texture0, fragTexCoord.xy);
    bool in_shape = in_frame_outline(st) || in_arrow_body(st) || in_arrow_head(st);
    vec4 color_fg = vec4(float(in_shape) * fragColor.rgb, 1.);
    vec4 color = in_shape ? color_fg : color_bg;

    finalColor = color;
}
