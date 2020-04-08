struct VS_INPUT
{
    float4 pos: POSITION;
    float4 pos1: POSITION1;
    float3 color: COLOR;
};

struct VS_OUTPUT
{
    float4 pos: SV_POSITION;
    float3 color: COLOR;
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

    output.color = input.color;

    //if (output.pos.y > 0 && output.pos.y < 1)
    //{
    //    output.pos.x += 0.25f;
    //}
    return output;
}
