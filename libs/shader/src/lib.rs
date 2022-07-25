pub mod dir_light_bump_map;
pub mod directional_light;
pub mod point_light;
pub mod skybox;

use engine::math::{Matrix4x4, Vector4d};

pub use dir_light_bump_map::DirLightBumpMap;
pub use directional_light::DirectionalLight;
pub use point_light::PointLight;
pub use skybox::Skybox;

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
