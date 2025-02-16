#version 330 core

in vec2 text_coord;
in vec4 vertex_color;
in vec3 normal;
in vec3 frag_pos;

out vec4 frag_color;

uniform sampler2D texture_one;
uniform sampler2D texture_two;

uniform int is_texture_vert = 0;
uniform int is_multi_texture = 0;
uniform int is_color_vert = 0;
uniform vec4 color = vec4(1.0);

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;    
    float shininess;
}; 

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

//view == camera
uniform vec3 view_pos = vec3(0.0);

uniform int is_light = 0;
uniform Material material;
uniform Light light;

vec3 calculate_light();

void main()
{
    vec4 our_color = color;
    if (is_color_vert == 1) {
        our_color = vertex_color;
    }

    if (is_texture_vert == 1 && is_light == 0) {
        if (is_multi_texture == 1) {
            frag_color = mix(texture(texture_one, text_coord), texture(texture_two, text_coord), 0.2) * our_color;
        } else {
            frag_color = texture(texture_one, text_coord) * our_color;
        }
    } else {
        frag_color = our_color;
    }

    if (is_light == 1) {
        vec3 light_result = calculate_light();
        frag_color = vec4(light_result, 1.0) * frag_color;
    }
}

vec3 calculate_light() {
    vec3 ambient = vec3(1.0);
    if (is_texture_vert == 1) {
        ambient = light.ambient * texture(texture_one, text_coord).rgb;
    } else {
        ambient = light.ambient * material.ambient;
    }

    // diffuse
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light.position - frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);

    vec3 diffuse = vec3(1.0);
    if (is_texture_vert == 1) {
        diffuse = light.diffuse * diff * texture(texture_one, text_coord).rgb;
    } else {
        diffuse = light.diffuse * (diff * material.diffuse);
    }

    // specular
    vec3 view_dir = normalize(view_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    vec3 specular = vec3(1.0);
    if (is_multi_texture == 1) {
        specular = light.specular * spec * texture(texture_two, text_coord).rgb;
    } else {
        specular = light.specular * (spec * material.specular);
    }

    return ambient + diffuse + specular;
}