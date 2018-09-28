#version 140

in vec3 position;
in vec2 texcoords;

out vec2 _tex_coords;

uniform mat4 ortho_matrix;

void main() {
    gl_Position =ortho_matrix * vec4(position, 1.0);
    _tex_coords = texcoords;
}