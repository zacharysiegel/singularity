#version 330

#ifdef GL_ES
precision mediump float;
#endif// GL_ES

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

const int kernel_size = 2;
const int fan = 4;
const int sample_count = 1 + kernel_size * fan;
const float pi_2 = 6.2831853072;
const float d_theta = pi_2 / fan;
const float d_r = 1.5;

void main() {
    vec4 sum = texture(u_sampler0, fragTexCoord);

    // Theta should start within the vertical line
    for (float theta = pi_2 / 4.; theta < pi_2 * 5. / 4.; theta += d_theta) {
        for (float r = d_r; r <= kernel_size * d_r + d_r / 1000.; r += d_r) {
            vec2 offset = vec2(float(r * cos(theta)), float(r * sin(theta))) / u_resolution;
            sum += texture(u_sampler0, fragTexCoord + offset);
        }
    }

    vec4 avg = sum / float(sample_count);
    avg.a = 1.;

    finalColor = avg * colDiffuse;
}
