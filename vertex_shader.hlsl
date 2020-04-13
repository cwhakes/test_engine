struct VS_INPUT
{
    float4 pos: POSITION0;
    float2 tex_coord: TEXCOORD0;
};

struct VS_OUTPUT
{
    float4 pos: SV_POSITION;
    float2 tex_coord: TEXCOORD0;
};

cbuffer constant: register(b0)
{
    row_major float4x4 m_world;
    row_major float4x4 m_view;
    row_major float4x4 m_proj;
    unsigned int m_time;
};

VS_OUTPUT vsmain( VS_INPUT input )
{   
    VS_OUTPUT output = (VS_OUTPUT)0;
//    output.pos = lerp(input.pos, input.pos1, (1.0f+cos(m_time/1000.0f))/2.0);

// World space
    output.pos = mul(input.pos, m_world);
// View space
    output.pos = mul(output.pos, m_view);
// Projection space
    output.pos = mul(output.pos, m_proj);

    output.tex_coord = input.tex_coord;

    return output;
}
