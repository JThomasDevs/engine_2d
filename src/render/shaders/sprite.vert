#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 tex_coords;

uniform vec2 sprite_position;
uniform vec2 sprite_size;

out vec2 TexCoords;

void main() {
    vec2 world_pos = sprite_position + position * sprite_size;
    gl_Position = vec4(world_pos, 0.0, 1.0);
    TexCoords = tex_coords;
}
