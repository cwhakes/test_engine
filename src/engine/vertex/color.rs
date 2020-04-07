use super::Vertex;

use std::convert;
use std::ffi::CStr;

use winapi::shared::dxgiformat;
use winapi::um::d3d11;

#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Vertex for Color {
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
        let semantic_name = CStr::from_bytes_with_nul(b"COLOR\0").unwrap();

        let desc = d3d11::D3D11_INPUT_ELEMENT_DESC {
            SemanticName: semantic_name.as_ptr(),
            SemanticIndex: 0,
            Format: dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: offset as u32,
            InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        };

        Box::new(Some(desc).into_iter())
    }
}

impl convert::From<[f32; 3]> for Color {
    fn from(array: [f32; 3]) -> Self {
        Color {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}
