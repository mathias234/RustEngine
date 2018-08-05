#version 140

uniform vec3 light_dir;
uniform float ambient_light;
uniform sampler2D diffuse;
uniform sampler2D normal_map;
uniform vec3 view_pos;

in vec3 _normal;
in vec2 _texcoord;
in vec3 _frag_pos;
in mat3 _tbn_matrix;

out vec4 result;

vec4 CalcLight(vec3 direction, vec3 normal, vec3 worldPos)
{
    float specularIntensity = 2.0;
    float specularPower = 32;
    float lightIntensity = 1.0;

	float diffuseFactor = clamp(dot(normal, -direction), 0.0, 1.0);
    
    vec4 diffuseColor = vec4(0,0,0,0);
    vec4 specularColor = vec4(0,0,0,0);
    
    if(diffuseFactor > 0)
    {
        diffuseColor = vec4(1.0, 1.0, 1.0, 1.0) * lightIntensity * diffuseFactor;
        
        vec3 directionToEye = normalize(view_pos - worldPos);
        vec3 reflectDirection = normalize(reflect(direction, normal));
        float specularFactor = dot(directionToEye, reflectDirection);
        specularFactor = pow(specularFactor, specularPower);
        
        if(specularFactor > 0)
        {
            specularColor = vec4(1.0, 1.0, 1.0, 1.0) * specularIntensity * specularFactor;
        }
    }
    
    return diffuseColor + specularColor;
}

void main() {
    vec4 diffuseTex = texture(diffuse, _texcoord);

	vec3 normal = texture(normal_map, _texcoord).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    normal = normalize(_tbn_matrix * normal);

    result = diffuseTex * (ambient_light + CalcLight(light_dir, normal, _frag_pos));            
}