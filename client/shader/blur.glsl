#ifdef GL_ES
precision mediump float;
#endif // GL_ES

uniform u_dimensions;

void main() {
    vec2 st = gl_FragCoord.xy/u_dimensions;

    glFragColor = vec4();
}
