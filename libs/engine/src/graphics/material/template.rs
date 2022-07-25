/// Trait used to show that a struct is able to be used as input for a vertex shader
pub trait Template {
    const PIXEL_SHADER_PATH: &'static str;
    const VERTEX_SHADER_PATH: &'static str;

    type Environment;
}
