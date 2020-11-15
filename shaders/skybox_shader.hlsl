


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

cbuffer constant1: register(b2)
{
    float3 color;
};

float4 psmain( PS_INPUT input ) : SV_Target
{      
    float3 tex = Texture.Sample(TextureSampler, 1.0 - input.tex_coord);

    return float4(tex, 1.0);
}