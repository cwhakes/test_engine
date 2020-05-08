


Texture2D Texture: register(t0);
sampler TextureSampler: register(s0);

struct PS_INPUT
{
    float4 pos: SV_POSITION;
    float2 tex_coord: TEXCOORD0;
    float3 normal: NORMAL0;
    float3 cam_dir: CAMDIR;
};

cbuffer constant: register(b0)
{
    row_major float4x4 m_view;
    row_major float4x4 m_proj;
    float4 m_light_dir;
    float4 m_camera_pos;
};

cbuffer constant1: register(b1)
{
    row_major float4x4 m_world;
};

float4 psmain( PS_INPUT input ) : SV_Target
{      
    //float3 tex = Texture.Sample(TextureSampler, input.tex_coord * 0.5);
    float3 tex = float3(1.0, 1.0, 1.0);

    float3 ka = 0.1 * tex;
    float3 ia = float3(1.0, 1.0, 1.0);
    float3 ambient_light = ka * ia;

    float3 kd = 0.7 * tex;
    float3 id = float3(1.0, 1.0, 1.0);
    float amount_diffuse_light = max(0.0, dot(m_light_dir.xyz, input.normal));
    float3 diffuse_light = kd * amount_diffuse_light * id;

    float ks = 0.3;
    float is = float3(1.0, 1.0, 1.0);
    float3 reflected_light = reflect(m_light_dir, input.normal);
    float shininess = 30.0;
    float3 amount_specular_light = pow(max(0.0, dot(reflected_light, input.cam_dir)), shininess);
    float3 specular_light = ks * amount_specular_light * is;

    float3 light = ambient_light + diffuse_light + specular_light;

    return float4(light, 1.0);
}