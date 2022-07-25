use engine::graphics::material;

pub struct Skybox;

impl material::Template for Skybox {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\skybox\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\skybox\\vertex_shader.hlsl";

    type Environment = super::Environment;
}
