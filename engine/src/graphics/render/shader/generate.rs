macro_rules! shader_generate {
    (unsafe {
        $name: ident,
        $interface: ty,
        $create_shader: ident,
        $set_shader: ident,
        $set_shader_resource: ident,
        $set_sampler: ident,
        $set_constant_buffer: ident,
        $entry_point: expr,
        $target: expr
    }) => {
        pub enum $name {}

        impl ShaderType for $name {
            type ShaderInterface = $interface;

            fn create_shader(device: &Device, bytecode: &[u8]) -> error::Result<NonNull<Self::ShaderInterface>> {
                unsafe {
                    get_output(|shader| {
                        device.as_ref().$create_shader(
                            bytecode.as_ptr() as *const _,
                            bytecode.len(),
                            std::ptr::null_mut(),
                            shader,
                        )
                    })
                }
            }

            fn set_shader(context: &Context, shader: &mut Self::ShaderInterface) {
                unsafe { context.as_ref().$set_shader(shader, std::ptr::null(), 0) }
            }

            fn set_textures(context: &Context, textures: &mut [crate::graphics::resource::texture::Texture]) {
                unsafe {
                    let texture_pointers: Vec<_> = textures.iter_mut()
                        .map(|tex| tex.resource_view_ptr())
                        .collect();
                    let sampler_pointers: Vec<_> = textures.iter_mut()
                        .map(|tex| tex.sampler_state_ptr())
                        .collect();
                    context.as_ref().$set_shader_resource(0, texture_pointers.len() as u32, texture_pointers.as_ptr());
                    context.as_ref().$set_sampler(0, sampler_pointers.len() as u32, sampler_pointers.as_ptr());
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
