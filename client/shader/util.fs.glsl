float in_box(vec2 st, float upper_bound) {
    float accumulator = 1.;
    float edge = 1. - upper_bound;
    float top = step(edge, 1. - st.y);
    float right = step(edge, 1. - st.x);

    accumulator *= top * right;
    return accumulator;
}

vec2 st(vec2 givenCoord, vec2 resolution, vec2 target_origin, vec2 target_dimensions) {
    vec2 st = givenCoord * resolution;
    st.y = resolution.y - st.y;
    st = (st - target_origin) / target_dimensions;
    return st;
}
