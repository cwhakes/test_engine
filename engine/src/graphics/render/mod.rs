mod constant_buffer;
mod context;
mod device;
mod index_buffer;
mod raster_state;
mod swapchain;
mod vertex_buffer;

pub use constant_buffer::ConstantBuffer;
pub use context::Context;
pub use device::Device;
pub use index_buffer::IndexBuffer;
use raster_state::RasterState;
pub use swapchain::{SwapChain, WindowState};
pub use vertex_buffer::VertexBuffer;

use crate::error;
use crate::graphics::resource::{Mesh, shader};
use crate::graphics::material::{CullMode, Material};
use crate::util::get_output2;

use std::ptr::null_mut;
use winapi::um::{d3d11, d3dcommon};

pub struct Render {
    device: Device,
    _feature_level: d3dcommon::D3D_FEATURE_LEVEL,
    context: Context,
    raster_front: RasterState,
    raster_back: RasterState,
}

const DRIVER_TYPES: [d3dcommon::D3D_DRIVER_TYPE; 3] = [
    d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
    d3dcommon::D3D_DRIVER_TYPE_WARP,
    d3dcommon::D3D_DRIVER_TYPE_REFERENCE,
];

const FEATURE_LEVELS: [d3dcommon::D3D_FEATURE_LEVEL; 1] = [
    d3dcommon::D3D_FEATURE_LEVEL_11_0
];

impl Render {
    pub fn new() -> error::Result<Render> {
        unsafe {
            let mut feature_level = Default::default();
            //Default to error
            let mut result = Err(error::Custom("No driver types specified".to_string()));

            for &driver_type in DRIVER_TYPES.iter() {
                result = get_output2(|ptr1, ptr2| {
                    d3d11::D3D11CreateDevice(
                        null_mut(),
                        driver_type,
                        null_mut(),
                        d3d11::D3D11_CREATE_DEVICE_DEBUG,
                        FEATURE_LEVELS.as_ptr(),
                        FEATURE_LEVELS.len() as u32,
                        d3d11::D3D11_SDK_VERSION,
                        ptr1,
                        &mut feature_level,
                        ptr2,
                    )
                });

                if result.is_ok() {
                    break;
                }
            }
            let (device, context) = result?;
            let device = Device::from_nonnull(device)?;
            let raster_front = RasterState::new_front(&device)?;
            let raster_back = RasterState::new_back(&device)?;

            Ok(Render {
                device,
                _feature_level: feature_level,
                context: Context::from_nonnull(context)?,
                raster_front,
                raster_back,
            })
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn immediate_context(&self) -> &Context {
        &self.context
    }

    pub fn set_material(&mut self, material: &mut Material) {
        for (idx, const_buff) in material.const_buffs.iter_mut().enumerate() {
            if let Some((const_buff, _)) = const_buff {
                self.context.set_constant_buffer(idx as u32, const_buff);
            }
        }

        match material.cull_mode {
            CullMode::Front => self.set_front_face_culling(),
            CullMode::Back => self.set_back_face_culling(),
        };

        self.context.set_shader(&mut material.vs);
        self.context.set_shader(&mut material.ps);
        self.context.set_textures::<shader::Pixel>(&mut *material.textures);

    }

    pub fn draw_mesh_and_materials(
        &mut self,
        mesh: &Mesh,
        materials: &mut [Material],
    ) {
        let mut mesh_inner = mesh.inner();
        for material_id in mesh_inner.material_ids.clone().iter() {

            if let Some (material) = materials.get_mut(material_id.id) {
                self.set_material(material);
            } else {
                // TODO: set default material
                println!("Missing material for: {:#?}", material_id.name);
                continue;
            };

            self.context.set_vertex_buffer(&mut mesh_inner.vertex_buffer);
            self.context.set_index_buffer(&mut mesh_inner.index_buffer);

            self.context.draw_indexed_triangle_list(
                material_id.len,
                material_id.offset,
                0,
            );
        }
    }

    pub fn set_front_face_culling(&mut self) {
        unsafe {
            self.context.as_ref().RSSetState(self.raster_front.as_mut());
        }
    }

    pub fn set_back_face_culling(&mut self) {
        unsafe {
            self.context.as_ref().RSSetState(self.raster_back.as_mut());
        }
    }
}
