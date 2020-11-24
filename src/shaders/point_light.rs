use engine::math::{Matrix4x4, Vector4d};

pub const PIXEL_SHADER_PATH: &str = "shaders\\point_light\\pixel_shader.hlsl";
pub const VERTEX_SHADER_PATH: &str = "shaders\\point_light\\vertex_shader.hlsl";

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
