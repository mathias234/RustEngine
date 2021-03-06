#version 140

in vec2 _tex_coords;

out vec4 result;

uniform sampler2D ui_texture;
uniform vec4 color;

void main() {
    vec4 diffuseTex = texture(ui_texture, _tex_coords);

    result = diffuseTex * color; 
}