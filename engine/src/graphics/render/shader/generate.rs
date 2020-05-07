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

            unsafe fn create_shader(device: &Device, bytecode: &[u8]) -> error::Result<*mut Self::ShaderInterface> {
                let mut shader = std::ptr::null_mut();
                device.as_ref().$create_shader(
                    bytecode.as_ptr() as *const _,
                    bytecode.len(),
                    std::ptr::null_mut(),
                    &mut shader,
                ).result()?;
                Ok(shader)
            }

            fn set_shader(context: &Context, shader: &mut Self::ShaderInterface) {
                unsafe { context.as_ref().$set_shader(shader, std::ptr::null(), 0) }
            }

            fn set_texture(context: &Context, texture: &mut crate::graphics::resource::texture::Texture) {
                unsafe {
                    context.as_ref().$set_shader_resource(0, 1, &texture.resource_view_ptr());
                }
            }

            fn set_constant_buffer<C>(context: &Context, index: u32, buffer: &mut ConstantBuffer<C>) {
                unsafe { context.as_ref().$set_constant_buffer(index, 1, &buffer.buffer_ptr()) }
            }

            const ENTRY_POINT: &'static str = $entry_point;
            const TARGET: &'static str = $target;
        }
    }
}
