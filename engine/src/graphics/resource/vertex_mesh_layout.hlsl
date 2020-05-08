struct VS_INPUT
{
    float4 pos: POSITION0;
    float2 tex_coord: TEXCOORD0;
    float3 normal: NORMAL0;
};

struct VS_OUTPUT
{

};

VS_OUTPUT vsmain( VS_INPUT input )
{   
    VS_OUTPUT output = (VS_OUTPUT)0;

    return output;
}
