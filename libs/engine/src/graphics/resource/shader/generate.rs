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
        #[derive(Clone)]
        pub enum $name {}

        impl ShaderType for $name {
            type ShaderInterface = $interface;

            fn create_shader(
                device: &Device,
                bytecode: &[u8],
            ) -> error::Result<NonNull<Self::ShaderInterface>> {
                unsafe {
                    get_output(|shader| {
                        device.as_ref().$create_shader(
                            bytecode.as_ptr().cast(),
                            bytecode.len(),
                            std::ptr::null_mut(),
                            shader,
                        )
                    })
                }
            }

            fn set_shader(context: &Context, shader: &Self::ShaderInterface) {
                unsafe {
                    // Should be safe; see parent. This shouldn't be mutated
                    let shader = shader as *const _ as *mut _;
                    context.as_ref().$set_shader(shader, std::ptr::null(), 0)
                }
            }

            fn set_textures(
                context: &Context,
                textures: &mut [Option<Arc<dyn $crate::graphics::material::Texture>>],
            ) {
                unsafe {
                    let texture_pointers: Vec<_> = textures
                        .iter_mut()
                        .flatten()
                        .map(|tex| tex.resource_view_ptr())
                        .collect();
                    let sampler_pointers: Vec<_> = textures
                        .iter_mut()
                        .flatten()
                        .map(|tex| tex.sampler_state_ptr())
                        .collect();
                    context.as_ref().$set_shader_resource(
                        0,
                        texture_pointers.len() as u32,
                        texture_pointers.as_ptr(),
                    );
                    context.as_ref().$set_sampler(
                        0,
                        sampler_pointers.len() as u32,
                        sampler_pointers.as_ptr(),
                    );
                }
            }

            fn set_constant_buffer<C: ?Sized>(
                context: &Context,
                index: u32,
                buffer: &mut ConstantBuffer<C>,
            ) {
                unsafe {
                    context
                        .as_ref()
                        .$set_constant_buffer(index, 1, &buffer.buffer_ptr())
                }
            }

            const ENTRY_POINT: &'static str = $entry_point;
            const TARGET: &'static str = $target;
        }
    };
}
