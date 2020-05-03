use super::Vertex;
use crate::math::Vector3d;

use std::{convert, ops};
use std::ffi::CStr;

use winapi::shared::dxgiformat;
use winapi::um::d3d11;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Normal(Vector3d);

impl Vertex for Normal {
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
        let semantic_name = CStr::from_bytes_with_nul(b"NORMAL\0").unwrap();

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

impl<T: Into<Vector3d>> convert::From<T> for Normal {
    fn from(vector: T) -> Self {
        Normal(vector.into())
    }
}

impl ops::Deref for Normal {
    type Target = Vector3d;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Normal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
