macro_rules! shader_generate {
    ($name: ident, $interface: ty,
        $create_shader: ident,
        $set_shader: ident,
        $set_shader_resource: ident,
        $set_constant_buffer: ident,
        $entry_point: expr,
        $target: expr
    ) => {
        pub enum $name {}

        impl ShaderType for $name {
            type ShaderInterface = $interface;

            unsafe fn create_shader(
                device: &d3d11::ID3D11Device,
                bytecode: *const std::ffi::c_void,
                bytecode_len: winapi::shared::basetsd::SIZE_T,
                shader: *mut *mut Self::ShaderInterface,
            ) {
                device.$create_shader(bytecode, bytecode_len, std::ptr::null_mut(), shader);
            }

            fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: &mut Self::ShaderInterface) {
                unsafe { context.$set_shader(shader, std::ptr::null(), 0) }
            }

            fn set_texture(context: &d3d11::ID3D11DeviceContext, texture: &mut crate::graphics::resource::texture::Texture) {
                unsafe {
                    context.$set_shader_resource(0, 1, &texture.resource_view_ptr());
                }
            }

            fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &mut crate::graphics::render::ConstantBuffer<C>) {
                unsafe { context.$set_constant_buffer(0, 1, &buffer.buffer_ptr()) }
            }

            const ENTRY_POINT: &'static str = $entry_point;
            const TARGET: &'static str = $target;
        }
    }
}
