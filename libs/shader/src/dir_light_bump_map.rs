use engine::graphics::material;

pub struct DirLightBumpMap;

impl material::Template for DirLightBumpMap {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\dir_light_bump_map\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\dir_light_bump_map\\vertex_shader.hlsl";

    type Environment = super::Environment;
}
