use engine::graphics::material;
use engine::math::{Matrix4x4, Vector4d};

pub struct Skybox;

impl material::Template for Skybox {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\skybox\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\skybox\\vertex_shader.hlsl";

    type Environment = Environment;
}

#[derive(Default, Debug)]
#[repr(C, align(16))]
pub struct Environment {
    pub view: Matrix4x4,
    pub proj: Matrix4x4,

    pub light_dir: Vector4d,
    pub camera_pos: Vector4d,
    pub light_pos: Vector4d,

    pub light_rad: f32,
    pub time: f32,
}
