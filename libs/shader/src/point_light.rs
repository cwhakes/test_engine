use engine::graphics::material;

pub struct PointLight;

impl material::Template for PointLight {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\point_light\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\point_light\\vertex_shader.hlsl";

    type Environment = super::Environment;
}
