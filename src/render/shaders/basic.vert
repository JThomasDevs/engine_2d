#version 330 core
layout (location = 0) in vec2 position;

uniform vec2 rect_position;
uniform vec2 rect_size;

void main() {
    vec2 world_pos = rect_position + position * rect_size;
    gl_Position = vec4(world_pos, 0.0, 1.0);
}
