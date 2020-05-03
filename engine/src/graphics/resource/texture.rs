use crate::prelude::*;

use super::{Resource, ResourceManager};

use crate::error;
use crate::graphics::render::Device;

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
    fn load_resource_from_file(device: &Device, path: &Path) -> error::Result<Self> {
        unsafe {
            let image = Reader::open(path)?.decode()?.to_rgba();

            let mut sample_desc = dxgitype::DXGI_SAMPLE_DESC::default();
            sample_desc.Count = 1;
            sample_desc.Quality = 0;

            let mut desc = d3d11::D3D11_TEXTURE2D_DESC::default();
            desc.Width = image.width();
            desc.Height = image.height();
            desc.MipLevels = 1;
            desc.ArraySize = 1;
            desc.Format = dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
            desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            desc.SampleDesc = sample_desc;
            desc.BindFlags = d3d11::D3D11_BIND_SHADER_RESOURCE;
            desc.CPUAccessFlags = 0;
            desc.MiscFlags = 0;
            
            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.SysMemPitch = image.sample_layout().height_stride.max(
                image.sample_layout().width_stride
            ) as u32;
            let buffer: Vec<u8> = image.into_raw();
            data.pSysMem = buffer.as_ptr() as *const _;

            let mut texture = ptr::null_mut();
            device.as_ref().CreateTexture2D(
                &desc,
                &data,
                &mut texture,
            ).result()?;
            let texture = NonNull::new(texture).ok_or(null_ptr_err!())?;
            
            drop(buffer);

            let mut resource_view = ptr::null_mut();
            device.as_ref().CreateShaderResourceView(
                &**texture.as_ref() as *const d3d11::ID3D11Resource as *mut _,
                ptr::null(),
                &mut resource_view,
            ).result()?;
            let resource_view = NonNull::new(resource_view).ok_or(null_ptr_err!())?;

            Ok( Texture(Arc::new(TextureInner {
                texture,
                resource_view,
            })))
        }
    }
}

impl Texture {
    pub fn resource_view_ptr(&mut self) -> *mut d3d11::ID3D11ShaderResourceView {
        //TODO Fix Shared Mutability
        self.0.as_ref().resource_view.as_ptr()
    }
}

impl AsRef<d3d11::ID3D11Texture2D> for Texture {
    fn as_ref(&self) -> &d3d11::ID3D11Texture2D {
        unsafe {
            self.0.as_ref().texture.as_ref()
        }
    }
}

struct TextureInner {
    texture: NonNull<d3d11::ID3D11Texture2D>,
    resource_view: NonNull<d3d11::ID3D11ShaderResourceView>,
}

//TODO Verify
unsafe impl Send for TextureInner {}
unsafe impl Sync for TextureInner {}

impl Drop for TextureInner {
    fn drop(&mut self) {
        unsafe {
            self.texture.as_ref().Release();
            self.resource_view.as_ref().Release();
        }
    }
}