use crate::graphics::material;
use crate::math::Rect;

use crate::error;
use crate::graphics::render::Device;
use crate::util::get_output;

use std::ptr::{self, NonNull};

use winapi::shared::{dxgiformat, dxgitype};
use winapi::um::d3d11;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Flavor {
    #[default]
    Normal,
    RenderTarget,
    DepthStencil,
}

#[derive(Default)]
pub struct RenderedTexture {
    pub flavor: Flavor,
    texture: Option<NonNull<d3d11::ID3D11Resource>>,
    sampler_state: Option<NonNull<d3d11::ID3D11SamplerState>>,
    shader_res_view: Option<NonNull<d3d11::ID3D11ShaderResourceView>>,
    render_target_view: Option<NonNull<d3d11::ID3D11RenderTargetView>>,
    depth_stencil_view: Option<NonNull<d3d11::ID3D11DepthStencilView>>,
}

unsafe impl Send for RenderedTexture {}
unsafe impl Sync for RenderedTexture {}

impl RenderedTexture {
    pub fn new(rect: Rect<u32>, flavor: Flavor, device: &Device) -> error::Result<Self> {
        unsafe {
            let tex_desc = d3d11::D3D11_TEXTURE2D_DESC {
                Width: rect.width(),
                Height: rect.height(),
                MipLevels: 1,
                ArraySize: 1,
                Format: match flavor {
                    Flavor::Normal | Flavor::RenderTarget => dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                    Flavor::DepthStencil => dxgiformat::DXGI_FORMAT_D24_UNORM_S8_UINT,
                },
                Usage: d3d11::D3D11_USAGE_DEFAULT,

                SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },

                BindFlags: match flavor {
                    Flavor::Normal => d3d11::D3D11_BIND_SHADER_RESOURCE,
                    Flavor::RenderTarget => {
                        d3d11::D3D11_BIND_RENDER_TARGET | d3d11::D3D11_BIND_SHADER_RESOURCE
                    }
                    Flavor::DepthStencil => d3d11::D3D11_BIND_DEPTH_STENCIL,
                },
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };

            let texture = get_output(|ptr| {
                device
                    .as_ref()
                    .CreateTexture2D(&tex_desc, ptr::null_mut(), ptr)
            })?
            .cast::<d3d11::ID3D11Resource>();

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

            match flavor {
                Flavor::Normal => {
                    let srv = get_output(|ptr| {
                        device.as_ref().CreateShaderResourceView(
                            texture.as_ptr(),
                            ptr::null_mut(),
                            ptr,
                        )
                    })?;

                    Ok(Self {
                        flavor,
                        sampler_state: Some(sampler_state),
                        texture: Some(texture),
                        shader_res_view: Some(srv),
                        ..Self::default()
                    })
                }
                Flavor::RenderTarget => {
                    let srv = get_output(|ptr| {
                        device.as_ref().CreateShaderResourceView(
                            texture.as_ptr(),
                            ptr::null_mut(),
                            ptr,
                        )
                    })?;

                    let rtv = get_output(|ptr| {
                        device.as_ref().CreateRenderTargetView(
                            texture.as_ptr(),
                            ptr::null_mut(),
                            ptr,
                        )
                    })?;

                    Ok(Self {
                        flavor,
                        texture: Some(texture),
                        sampler_state: Some(sampler_state),
                        shader_res_view: Some(srv),
                        render_target_view: Some(rtv),
                        ..Self::default()
                    })
                }
                Flavor::DepthStencil => {
                    let dsv = get_output(|ptr| {
                        device.as_ref().CreateDepthStencilView(
                            texture.as_ptr(),
                            ptr::null_mut(),
                            ptr,
                        )
                    })?;

                    Ok(Self {
                        flavor,
                        sampler_state: Some(sampler_state),
                        texture: Some(texture),
                        depth_stencil_view: Some(dsv),
                        ..Self::default()
                    })
                }
            }
        }
    }
}

impl material::Texture for RenderedTexture {
    fn sampler_state_ptr(&self) -> *mut d3d11::ID3D11SamplerState {
        self.sampler_state.unwrap().as_ptr()
    }

    fn resource_view_ptr(&self) -> *mut d3d11::ID3D11ShaderResourceView {
        self.shader_res_view.unwrap().as_ptr()
    }
}

impl Drop for RenderedTexture {
    fn drop(&mut self) {
        let Self {
            texture,
            shader_res_view,
            render_target_view,
            depth_stencil_view,
            sampler_state,
            ..
        } = self;

        unsafe {
            texture.map(|t| t.as_ref().Release());
            shader_res_view.map(|s| s.as_ref().Release());
            render_target_view.map(|r| r.as_ref().Release());
            depth_stencil_view.map(|d| d.as_ref().Release());
            sampler_state.map(|s| s.as_ref().Release());
        }
    }
}
