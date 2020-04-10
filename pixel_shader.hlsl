struct PS_INPUT
{
    float4 pos: SV_POSITION;
    float3 color: COLOR;
    float3 color1: COLOR1;
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
    return float4(lerp(input.color, input.color1, (1.0f+cos(m_time/500.0f))/2.0), 1.0f );   
 }