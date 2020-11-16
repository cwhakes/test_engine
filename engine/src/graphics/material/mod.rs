
use crate::error::Result;
use crate::graphics::render::{ConstantBuffer, Device, Render};
use crate::graphics::render::shader::{self, Shader};
use crate::graphics::resource::Texture;
use std::any::{Any, TypeId};
use std::path::Path;

pub struct Material {
    pub vs: Shader<shader::Vertex>,
    pub ps: Shader<shader::Pixel>,
    pub const_buffs : Vec<Option<(ConstantBuffer<dyn Any + Send + Sync>, TypeId)>>,
    pub textures: Vec<Option<Texture>>,
    pub cull_mode: CullMode,
}

impl Material {
    pub fn new(device: &Device, vs: impl AsRef<Path>, ps: impl AsRef<Path>, cull_mode: CullMode) -> Result<Self> {

        let (vertex_shader, _) = device.new_shader::<shader::Vertex, _>(vs)?;
        let (pixel_shader, _) = device.new_shader::<shader::Pixel, _>(ps)?;

        Ok(Material {
            vs: vertex_shader,
            ps: pixel_shader,
            const_buffs: Vec::new(),
            textures: Vec::new(),
            cull_mode,
        })
    }

    pub fn add_texture(&mut self, texture: &Texture) -> usize {
        self.textures.push(Some(texture.clone()));
        self.textures.len() - 1
    }

    pub fn remove_texture(&mut self, idx: usize) {
        self.textures.get_mut(idx).map(|tex| *tex = None);
    }

    pub fn set_data<A: Any + Send + Sync>(&mut self, render: &Render, idx: usize, data: &mut A) -> Result<()> {
        if self.const_buffs.len() <= idx {
            self.const_buffs.resize_with(idx + 1, || None);
        }
        
        if let Some((mut const_buff, type_id)) = self.const_buffs[idx].take() {

            if TypeId::of::<A>() != type_id {
                let error = &*format!("Type Error: {:?} != {:?}", TypeId::of::<A>(), type_id);
                return Err(error.into())
            }
            let context = render.immediate_context();
            const_buff.update(context, data);
            //context.set_constant_buffer(idx as u32, &mut const_buff);
            self.const_buffs[idx] = Some((const_buff, type_id))

        } else {
            let const_buff = render.device().new_constant_buffer(data as &mut (dyn Any + Send + Sync))?;
            let type_id = TypeId::of::<A>();
            self.const_buffs[idx] = Some((const_buff, type_id));
        };

        Ok(())
    }
}

pub enum CullMode {
    Front,
    Back,
}
