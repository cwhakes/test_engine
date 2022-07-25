use engine::graphics::material;

pub struct DirectionalLight;

impl material::Template for DirectionalLight {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\directional_light\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\directional_light\\vertex_shader.hlsl";

    type Environment = super::Environment;
}
