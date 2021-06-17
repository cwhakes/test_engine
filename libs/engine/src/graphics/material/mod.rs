use crate::error::Result;
use crate::graphics::render::{ConstantBuffer, Render};
use crate::graphics::resource::shader::{self, Shader};
use crate::graphics::resource::Texture;
use crate::graphics::Graphics;
use std::any::{Any, TypeId};
use std::path::Path;

pub struct Material {
    pub vs: Shader<shader::Vertex>,
    pub ps: Shader<shader::Pixel>,
    pub const_buffs: Vec<Option<(ConstantBuffer<dyn Any + Send + Sync>, TypeId)>>,
    pub textures: Vec<Option<Texture>>,
    pub cull_mode: CullMode,
}

#[derive(Clone, Debug)]
pub enum CullMode {
    Front,
    Back,
}

impl Material {
    pub fn new(
        graphics: &mut Graphics,
        vs: impl AsRef<Path>,
        ps: impl AsRef<Path>,
    ) -> Result<Self> {
        let vertex_shader = graphics.get_vertex_shader_from_file(vs)?;
        let pixel_shader = graphics.get_pixel_shader_from_file(ps)?;

        Ok(Self {
            vs: vertex_shader,
            ps: pixel_shader,
            const_buffs: Vec::new(),
            textures: Vec::new(),
            cull_mode: CullMode::Back,
        })
    }

    pub fn with_frontface_culling(mut self) -> Self {
        self.cull_mode = CullMode::Front;
        self
    }

    pub fn add_texture(&mut self, texture: &Texture) -> usize {
        self.textures.push(Some(texture.clone()));
        self.textures.len() - 1
    }

    pub fn remove_texture(&mut self, idx: usize) {
        if let Some(tex) = self.textures.get_mut(idx) {
            *tex = None;
        }
    }

    pub fn set_data<A: Any + Send + Sync>(
        &mut self,
        render: &Render,
        idx: usize,
        data: &mut A,
    ) -> Result<()> {
        if self.const_buffs.len() <= idx {
            self.const_buffs.resize_with(idx + 1, || None);
        }

        if let Some((mut const_buff, type_id)) = self.const_buffs[idx].take() {
            if TypeId::of::<A>() != type_id {
                let error = &*format!("Type Error: {:?} != {:?}", TypeId::of::<A>(), type_id);
                return Err(error.into());
            }
            let context = render.immediate_context();
            const_buff.update(context, data);
            //context.set_constant_buffer(idx as u32, &mut const_buff);
            self.const_buffs[idx] = Some((const_buff, type_id))
        } else {
            let const_buff = render
                .device()
                .new_constant_buffer(data as &mut (dyn Any + Send + Sync))?;
            let type_id = TypeId::of::<A>();
            self.const_buffs[idx] = Some((const_buff, type_id));
        };

        Ok(())
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self {
            vs: self.vs.clone(),
            ps: self.ps.clone(),
            const_buffs: Vec::new(),
            textures: self.textures.clone(),
            cull_mode: self.cull_mode.clone(),
        }
    }
}
