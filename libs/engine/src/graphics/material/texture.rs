use winapi::um::d3d11;

pub trait Texture: Clone {
    fn sampler_state_ptr(&mut self) -> *mut d3d11::ID3D11SamplerState;
    fn resource_view_ptr(&mut self) -> *mut d3d11::ID3D11ShaderResourceView;
}
