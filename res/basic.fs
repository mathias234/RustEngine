#version 140

uniform vec3 light;
uniform float ambient_light;
uniform sampler2D diffuse;

in vec3 _normal;
in vec2 _texcoord;
out vec4 result;


void main() {
    float diffuseFactor = dot(_normal, -light);
    vec4 diffuseColor = vec4(0,0,0,0);
    
    if(diffuseFactor > 0)
    {
        diffuseColor = vec4(1.0, 1.0, 1.0, 1.0) * 1 * diffuseFactor;
    }

    vec4 diffuseTex = texture(diffuse, _texcoord);
    vec4 ambientLight = vec4(ambient_light, ambient_light, ambient_light, 1.0f);
    result = diffuseTex * (ambientLight + diffuseColor);            
}