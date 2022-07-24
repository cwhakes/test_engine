#[macro_use]
mod generate;

use crate::math::{Vector2d, Vector3d, Vector4d};

/// Re-export used in proc macro
pub use winapi::um::d3d11::D3D11_INPUT_ELEMENT_DESC;

use winapi::shared::dxgiformat;

use std::collections::HashMap;
use std::ffi::{CStr, CString};

use winapi::um::d3d11;

/// A trait reqired by `VertexBuffer`;
/// used to automatically generate layouts.
pub trait Vertex {
    /// Creates a layout description used by `CreateInputLayout` for shaders.
    /// Collect results into an array
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>>;
}

vertex_generate!(
    Color,
    Vector3d,
    b"COLOR\0",
    dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT
);
vertex_generate!(
    Normal,
    Vector3d,
    b"NORMAL\0",
    dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT
);
vertex_generate!(
    Tangent,
    Vector3d,
    b"TANGENT\0",
    dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT
);
vertex_generate!(
    BiNormal,
    Vector3d,
    b"BINormal\0",
    dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT
);
vertex_generate!(
    Position,
    Vector4d,
    b"POSITION\0",
    dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT
);
vertex_generate!(
    TexCoord,
    Vector2d,
    b"TEXCOORD\0",
    dxgiformat::DXGI_FORMAT_R32G32_FLOAT
);

/// `SemanticIndex` must be unique per `SemanticName`.
/// Import this trait and call `semantic_index_fix` before collecting descriptions into an array.
pub trait SemanticIndexFix: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC> {
    fn semantic_index_fix(self) -> SemanticIndexFixIter<Self>
    where
        Self: Sized,
    {
        SemanticIndexFixIter {
            iter: self,
            count: HashMap::new(),
        }
    }
}

impl<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> SemanticIndexFix for I {}

/// Created by `SemanticIndexFix`.
pub struct SemanticIndexFixIter<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
    iter: I,
    count: HashMap<CString, u32>,
}

impl<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> Iterator for SemanticIndexFixIter<I> {
    type Item = d3d11::D3D11_INPUT_ELEMENT_DESC;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mut desc| {
            // Extract the SemanticName
            let name = unsafe { CStr::from_ptr(desc.SemanticName).to_owned() };
            // Count how many times the name has been used
            let index = *self.count.get(&name).unwrap_or(&0);
            // Set the SemanticIndex
            desc.SemanticIndex = index;
            // And finally increment the count by 1
            self.count.insert(name, index + 1);
            desc
        })
    }
}
