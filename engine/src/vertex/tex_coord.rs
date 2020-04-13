use super::Vertex;
use crate::math::Vector2d;

use std::{convert, ops};
use std::ffi::CStr;

use winapi::shared::dxgiformat;
use winapi::um::d3d11;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct TexCoord(Vector2d);

impl Vertex for TexCoord {
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
        let semantic_name = CStr::from_bytes_with_nul(b"TEXCOORD\0").unwrap();

        let desc = d3d11::D3D11_INPUT_ELEMENT_DESC {
            SemanticName: semantic_name.as_ptr(),
            SemanticIndex: 0,
            Format: dxgiformat::DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: offset as u32,
            InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        };

        Box::new(Some(desc).into_iter())
    }
}

impl<T: Into<Vector2d>> convert::From<T> for TexCoord {
    fn from(vector: T) -> Self {
        TexCoord(vector.into())
    }
}

impl ops::Deref for TexCoord {
    type Target = Vector2d;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for TexCoord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
