use engine::graphics::material;
use engine::math::{Matrix4x4, Vector4d};

pub struct DirectionalLight;

impl material::Template for DirectionalLight {
    const PIXEL_SHADER_PATH: &'static str = "shaders\\directional_light\\pixel_shader.hlsl";
    const VERTEX_SHADER_PATH: &'static str = "shaders\\directional_light\\vertex_shader.hlsl";

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
