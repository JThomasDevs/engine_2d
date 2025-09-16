#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform sampler2D texture_sampler;
uniform vec3 tint_color;
uniform float alpha;

void main() {
    vec4 tex_color = texture(texture_sampler, TexCoords);
    FragColor = vec4(tex_color.rgb * tint_color, tex_color.a * alpha);
}
