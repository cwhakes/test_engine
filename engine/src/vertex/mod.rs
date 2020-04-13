mod color;
mod position;
mod tex_coord;

pub use color::Color;
pub use position::Position;
pub use tex_coord::TexCoord;

use std::collections::HashMap;
use std::ffi::{CStr, CString};

use winapi::um::d3d11;

/// A trait reqired by VertexBuffer;
/// used to automatically generate layouts.
pub trait Vertex {
    /// Creates a layout description used by CreateInputLayout for shaders.
    /// Collect results into an array
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>>;
}

/// SemanticIndexes must be unique per SemanticName.
/// Import this trait and call `semantic_index_fix` before collecting descriptions into an array.
pub trait SemanticIndexFix: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC> {
    fn semantic_index_fix(self) -> SemanticIndexFixIter<Self> where Self: Sized {
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
        self.iter.next().map( |mut desc| {
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
