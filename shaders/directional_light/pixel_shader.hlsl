Texture2D Color: register(t0);
sampler ColorSampler: register(s0);

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
    float4 color = Color.Sample(ColorSampler, float2(input.tex_coord.x, 1.0 - input.tex_coord.y));

    //Ambient
    float3 ka = 8.5;
    float3 ia = float3(0.09, 0.082, 0.082);
    ia *= (color.rgb);
    float3 ambient_light = ka * ia;

    //Diffuse
    float3 kd = 0.7;
    float amount_diffuse_light = dot(m_light_dir.xyz, input.normal);
    float3 id = float3(1.0, 1.0, 1.0);
    id *= (color.rgb);

    float3 diffuse_light = kd * id * amount_diffuse_light;

    //Specular
    float ks = 0.0;
    float3 is = float3(1.0, 1.0, 1.0);
    float3 reflected_light = reflect(m_light_dir.xyz, input.normal);
    float shininess = 30.0;
    float3 amount_specular_light = pow(max(0.0, dot(reflected_light, input.cam_dir)), shininess);
    float3 specular_light = ks * amount_specular_light * is;

    float3 light = ambient_light + diffuse_light + specular_light;

    return float4(light, 1.0);
}