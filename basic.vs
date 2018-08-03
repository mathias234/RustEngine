#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec3 normal;

out vec3 _normal;

void main() {
    mat4 mvp = persp_matrix * view_matrix * model_matrix;

    gl_Position = mvp * vec4(position, 1.0);
    
    _normal = (model_matrix * vec4(normal, 0.0)).xyz;
}