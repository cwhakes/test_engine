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
    unsigned int m_time;
};

VS_OUTPUT vsmain( VS_INPUT input )
{   
    VS_OUTPUT output = (VS_OUTPUT)0;
    output.pos = lerp(input.pos, input.pos1, (1.0f+cos(m_time/1000.0f))/2.0);
    output.color = input.color;

    if (output.pos.y > 0 && output.pos.y < 1)
    {
        output.pos.x += 0.25f;
    }
    return output;
}
