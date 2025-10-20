#version 330

#ifdef GL_ES
precision mediump float;
#endif // GL_ES

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 FragColor;

uniform vec2 u_dimensions;
uniform vec2 u_mouse;
uniform float u_time;
uniform sampler2D u_sampler0;

const int kernel_size = 1;
const float sample_count = pow(1 + kernel_size * 2, 2);

void main() {
    vec2 texelSize = 1.0 / u_dimensions;
    vec4 sum = vec4(0.);

    for (int x = -kernel_size; x <= kernel_size; x += 1) {
        for (int y = -kernel_size; y <= kernel_size; y += 1) {
            vec2 offset = vec2(float(x), float(y)) * texelSize;
            sum += texture(u_sampler0, fragTexCoord + offset);
        }
    }

    FragColor = sum / sample_count;
}
