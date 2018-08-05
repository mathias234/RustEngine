#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec3 normal;
in vec3 tangent;
in vec2 texcoord;

out vec2 _texcoord;
out vec3 _normal;
out vec3 _frag_pos;
out mat3 _tbn_matrix;

void main() {
    mat4 mvp = persp_matrix * view_matrix * model_matrix;

    gl_Position = mvp * vec4(position, 1.0);
    
    _frag_pos = vec3(model_matrix * vec4(position, 1.0));

    _normal = (model_matrix * vec4(normal, 0.0)).xyz;
    _texcoord = texcoord;

    vec3 n = normalize((model_matrix * vec4(normal, 0.0)).xyz);
	vec3 t = normalize((model_matrix * vec4(tangent, 0.0)).xyz);
	
	t = normalize(t - dot(t, n) * n);
	
	vec3 biTangent = cross(t, n);
	
	_tbn_matrix = mat3(t, biTangent, n);
}