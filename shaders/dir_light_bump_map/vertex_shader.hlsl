struct VS_INPUT
{
    float4 pos: POSITION0;
    float2 tex_coord: TEXCOORD0;
    float3 tangent: TANGENT0;
    float3 binormal: BINORMAL0;
    float3 normal: NORMAL0;
};

struct VS_OUTPUT
{
    float4 pos: SV_POSITION;
    float2 tex_coord: TEXCOORD0;
    float3 cam_dir: CAMDIR;
    row_major float3x3 tbn: TBN;
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

VS_OUTPUT vsmain( VS_INPUT input )
{   
    VS_OUTPUT output = (VS_OUTPUT)0;
//    output.pos = lerp(input.pos, input.pos1, (1.0f+cos(m_time/1000.0f))/2.0);

// World space
    output.pos = mul(input.pos, m_world);
    output.cam_dir = normalize(output.pos.xyz - m_camera_pos.xyz); 
// View space
    output.pos = mul(output.pos, m_view);
// Projection space
    output.pos = mul(output.pos, m_proj);

    output.tex_coord = input.tex_coord;
    
    output.tbn[0] = normalize(mul(input.tangent, m_world));
    output.tbn[1] = normalize(mul(input.binormal, m_world));
    output.tbn[2] = normalize(mul(input.normal, m_world));

    return output;
}
