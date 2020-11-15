


Texture2D Texture: register(t0);
sampler TextureSampler: register(s0);

struct PS_INPUT
{
    float4 pos: SV_POSITION;
    float2 tex_coord: TEXCOORD0;
    float3 normal: NORMAL0;
    float3 world_pos: TEXCOORD1;
};

cbuffer constant: register(b0)
{
    row_major float4x4 m_view;
    row_major float4x4 m_proj;
    float4 m_light_dir;
    float4 m_camera_pos;
    float4 m_light_pos;
    float m_light_rad;
    float time;
};

cbuffer constant1: register(b1)
{
    row_major float4x4 m_world;
};

cbuffer constant1: register(b2)
{
    float3 color;
};

float4 psmain( PS_INPUT input ) : SV_Target
{      
    float3 tex = Texture.Sample(TextureSampler, (1.0 - input.tex_coord) * 2.0);
    //float3 tex = float3(1.0, 1.0, 1.0);
    //float3 tex = color;

    float3 light_dir = normalize(m_light_pos.xyz - input.world_pos.xyz);
    float light_len = length(m_light_pos.xyz - input.world_pos.xyz);
    float fade = max(0, light_len - m_light_rad);

    float const_func = 1.0;
    float linear_func = 2.0;
    float quad_func = 2.0;

    float attenuation = const_func + linear_func * fade + quad_func * fade * fade;

    //Ambient
    float3 ka = 1.5;
    float3 ia = float3(0.09, 0.082, 0.082);
    ia *= tex;
    float3 ambient_light = ka * ia;

    //Diffuse
    float3 kd = 0.7;
    float amount_diffuse_light = max(0.0, dot(light_dir, input.normal));
    float3 id = float3(1.0, 1.0, 1.0);
    id *= tex;
    float3 diffuse_light = (kd * amount_diffuse_light * id) / attenuation;

    //Specular
    float ks = 1.0;
    float3 cam_dir = normalize(input.world_pos.xyz - m_camera_pos.xyz); 
    float3 is = float3(1.0, 1.0, 1.0);
    float3 reflected_light = reflect(light_dir, input.normal);
    float shininess = 30.0;
    float3 amount_specular_light = pow(max(0.0, dot(reflected_light, cam_dir)), shininess);
    float3 specular_light = (ks * amount_specular_light * is) / attenuation;

    float3 light = ambient_light + diffuse_light + specular_light;

    return float4(light, 1.0);
}