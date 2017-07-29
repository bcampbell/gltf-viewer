#version 330 core
out vec4 FragColor;

// in vec3 Normal;
in vec4 Tangent;
in vec2 TexCoords_0;
in vec2 TexCoords_1;
in vec3 Color;

uniform sampler2D base_color_texture;
uniform vec3 base_color_factor;

void main()
{
    FragColor = texture(base_color_texture, TexCoords_0);
    // FragColor = vec4(0.800000011920929, 0.0, 0.0, 1.0);
    // FragColor = vec4(Normal, 1.0);
    // FragColor = vec4(Tangent.xyz, 1.0);
    // FragColor = vec4(TexCoords_0, 0.0, 1.0);
    // FragColor = vec4(TexCoords_1, 0,0, 1.0);
    // FragColor = vec4(Color, 1.0);
}
