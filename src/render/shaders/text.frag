#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform sampler2D text_texture;
uniform vec3 text_color;
uniform float alpha;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text_texture, TexCoords).a);
    FragColor = vec4(text_color, 1.0) * sampled * alpha;
}
