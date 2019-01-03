#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;
uniform uint id;

in vec3 position;
flat out uint v_id;

void main() {
    mat4 mvp = persp_matrix * view_matrix * model_matrix;

    gl_Position = mvp * vec4(position, 1.0);
    v_id = id;
}