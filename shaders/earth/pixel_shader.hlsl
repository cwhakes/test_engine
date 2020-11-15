


Texture2D EarthColor: register(t0);
sampler EarthColorSampler: register(s0);

Texture2D EarthSpec: register(t1);
sampler EarthSpecSampler: register(s1);

Texture2D Clouds: register(t2);
sampler CloudsSampler: register(s2);

Texture2D EarthNight: register(t3);
sampler EarthNightSampler: register(s3);


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
    float cloud_offset;
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
    float3 tex = float3(1.0, 1.0, 1.0);
    float4 earth_color = EarthColor.Sample(EarthColorSampler, 1.0 - input.tex_coord);
    float4 earth_spec = EarthSpec.Sample(EarthSpecSampler, 1.0 - input.tex_coord);
    float4 clouds = Clouds.Sample(CloudsSampler, 1.0 - input.tex_coord + float2(cloud_offset,0));
    float4 earth_night = EarthNight.Sample(EarthNightSampler, 1.0 - input.tex_coord);

    float3 ka = 1.5;
    float3 ia = float3(0.09, 0.082, 0.082);
    ia *= earth_color.rgb + clouds.rgb;
    float3 ambient_light = ka * ia;

    float3 kd = tex;

    float3 id_day = float3(1.0, 1.0, 1.0);
    id_day *= (earth_color.rgb + clouds.rgb);

    float3 id_night = float3(1.0, 1.0, 1.0);
    id_night *= (earth_night.rgb + clouds.rgb * 0.03);

    float amount_diffuse_light = dot(m_light_dir.xyz, input.normal);

    float3 id = lerp(id_night, id_day, (amount_diffuse_light + 1.0) / 2.0);
    
    float3 diffuse_light = kd * id;

    float ks = earth_spec.r + clouds.r;
    float3 is = float3(0.3, 0.3, 0.3);
    float3 reflected_light = reflect(m_light_dir, input.normal);
    float shininess = 10.0;
    float3 amount_specular_light = pow(max(0.0, dot(reflected_light, input.cam_dir)), shininess);
    float3 specular_light = ks * amount_specular_light * is;

    float3 light = ambient_light + diffuse_light + specular_light;

    return float4(light, 1.0);
}