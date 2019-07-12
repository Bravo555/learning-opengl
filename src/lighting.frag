#version 330 core

in vec3 fragNormal;
in vec3 fragPos;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 cameraPosition;

out vec4 color;

void main() {
    // ambient
    float ambientStrength = 0.05;
    vec3 ambient = ambientStrength * lightColor;

    // diffuse
    vec3 norm = normalize(fragNormal);
    vec3 lightDirection = normalize(lightPos - fragPos);
    float diff = max(dot(norm, lightDirection), 0.0);
    vec3 diffuse = diff * lightColor;

    // specular
    float specularStrength = 0.7;
    vec3 viewDir = normalize(cameraPosition - fragPos);
    vec3 reflectDir = reflect(-lightDirection, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 64);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    color = vec4(result, 1.0);
}
