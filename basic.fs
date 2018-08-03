#version 140

uniform vec3 light;
uniform float ambient_light;

in vec3 _normal;
out vec4 result;


void main() {
    float diffuseFactor = dot(_normal, -light);
    vec4 diffuseColor = vec4(0,0,0,0);
    
    if(diffuseFactor > 0)
    {
        diffuseColor = vec4(1.0, 1.0, 1.0, 1.0) * 1 * diffuseFactor;
    }

    result = vec4(ambient_light, ambient_light, ambient_light, 1.0f) + diffuseColor;            
}