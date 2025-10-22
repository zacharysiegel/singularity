#version 330

#ifdef GL_ES
precision mediump float;
#endif // GL_ES

in vec2 fragTexCoord;
in vec4 fragColor;
in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform vec2 u_dimensions;
uniform vec2 u_mouse;
uniform float u_time;

uniform sampler2D u_sampler0;
uniform vec4 colDiffuse;

const int kernel_size = 6;
const float sample_count = pow(1. + kernel_size * 2., 2.);

void main() {
    vec2 st = fragTexCoord.st;
    vec4 sum = vec4(0.);

    for (int x = -kernel_size; x <= kernel_size; x += 1) {
        for (int y = -kernel_size; y <= kernel_size; y += 1) {
            vec2 offset = vec2(float(x), float(y)) / 1000.;
            sum += texture(u_sampler0, st + offset);
        }
    }

    vec4 avg = sum / sample_count;
    avg.a = 1.;

    finalColor = avg * colDiffuse;
}
