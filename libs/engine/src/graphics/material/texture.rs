use winapi::um::d3d11;

pub trait Texture: Send + Sync {
    fn sampler_state_ptr(&self) -> *mut d3d11::ID3D11SamplerState;
    fn resource_view_ptr(&self) -> *mut d3d11::ID3D11ShaderResourceView;
}
