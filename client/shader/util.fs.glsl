float in_box(vec2 st, float upper_bound) {
    float accumulator = 1.;
    float edge = 1. - upper_bound;
    float top = step(edge, 1. - st.y);
    float right = step(edge, 1. - st.x);

    accumulator *= top * right;
    return accumulator;
}
