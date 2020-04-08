struct PS_INPUT
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

 float4 psmain( PS_INPUT input ) : SV_Target
 {
        return float4(input.color, 1.0f );   
 }