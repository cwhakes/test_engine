use super::{Resource, ResourceManager};

use crate::error;
use crate::graphics::material;
use crate::graphics::render::Device;
use crate::util::get_output;

use std::path::Path;
use std::ptr::{self, NonNull};
use std::sync::Arc;

use image::io::Reader;
use winapi::shared::{dxgiformat, dxgitype};
use winapi::um::d3d11;

pub type TextureManager = ResourceManager<Texture>;

#[derive(Clone)]
pub struct Texture(Arc<TextureInner>);

impl Resource for Texture {
    fn load_resource_from_file(device: &Device, path: impl AsRef<Path>) -> error::Result<Self> {
        unsafe {
            let image = Reader::open(path.as_ref())?.decode()?.to_rgba8();
            let sample_desc = dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            };

            let desc = d3d11::D3D11_TEXTURE2D_DESC {
                Width: image.width(),
                Height: image.height(),
                MipLevels: 1,
                ArraySize: 1,
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                Usage: d3d11::D3D11_USAGE_DEFAULT,
                SampleDesc: sample_desc,
                BindFlags: d3d11::D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };

            let pitch = image
                .sample_layout()
                .height_stride
                .max(image.sample_layout().width_stride) as u32;
            let buffer: Vec<u8> = image.into_raw();
            let data = d3d11::D3D11_SUBRESOURCE_DATA {
                pSysMem: buffer.as_ptr().cast(),
                SysMemPitch: pitch,
                ..Default::default()
            };
            let texture = get_output(|ptr| device.as_ref().CreateTexture2D(&desc, &data, ptr))?;
            drop(buffer);

            let sampler_desc = d3d11::D3D11_SAMPLER_DESC {
                AddressU: d3d11::D3D11_TEXTURE_ADDRESS_WRAP,
                AddressV: d3d11::D3D11_TEXTURE_ADDRESS_WRAP,
                AddressW: d3d11::D3D11_TEXTURE_ADDRESS_WRAP,
                Filter: d3d11::D3D11_FILTER_ANISOTROPIC,
                MinLOD: 0.0,
                MaxLOD: 1.0,
                ..d3d11::D3D11_SAMPLER_DESC::default()
            };

            let sampler_state =
                get_output(|ptr| device.as_ref().CreateSamplerState(&sampler_desc, ptr))?;

            let resource_view = get_output(|ptr| {
                device.as_ref().CreateShaderResourceView(
                    &**texture.as_ref() as *const d3d11::ID3D11Resource as *mut _,
                    ptr::null(),
                    ptr,
                )
            })?;

            Ok(Self(Arc::new(TextureInner {
                texture,
                sampler_state,
                resource_view,
            })))
        }
    }
}

impl material::Texture for Texture {
    fn sampler_state_ptr(&mut self) -> *mut d3d11::ID3D11SamplerState {
        //TODO Fix Shared Mutability
        self.0.as_ref().sampler_state.as_ptr()
    }

    fn resource_view_ptr(&mut self) -> *mut d3d11::ID3D11ShaderResourceView {
        //TODO Fix Shared Mutability
        self.0.as_ref().resource_view.as_ptr()
    }
}

impl AsRef<d3d11::ID3D11Texture2D> for Texture {
    fn as_ref(&self) -> &d3d11::ID3D11Texture2D {
        unsafe { self.0.as_ref().texture.as_ref() }
    }
}

struct TextureInner {
    texture: NonNull<d3d11::ID3D11Texture2D>,
    sampler_state: NonNull<d3d11::ID3D11SamplerState>,
    resource_view: NonNull<d3d11::ID3D11ShaderResourceView>,
}

//TODO Verify
unsafe impl Send for TextureInner {}
unsafe impl Sync for TextureInner {}

impl Drop for TextureInner {
    fn drop(&mut self) {
        unsafe {
            self.texture.as_ref().Release();
            self.sampler_state.as_ref().Release();
            self.resource_view.as_ref().Release();
        }
    }
}
