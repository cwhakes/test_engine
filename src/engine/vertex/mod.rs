mod color;
mod position;

pub use color::Color;
pub use position::Position;

use std::collections::HashMap;
use std::ffi::{CStr, CString};

use winapi::um::d3d11;

pub trait Vertex {
    fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>>;
}

pub trait SemanticIndexFix: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC> {
    fn semantic_index_fix(self) -> SemanticIndexFixIter<Self> where Self: Sized {
        SemanticIndexFixIter {
            iter: self,
            count: HashMap::new(),
        }
    }
}

impl<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> SemanticIndexFix for I {}

pub struct SemanticIndexFixIter<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
    iter: I, 
    count: HashMap<CString, u32>,
}

impl<I: Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> Iterator for SemanticIndexFixIter<I> {
    type Item = d3d11::D3D11_INPUT_ELEMENT_DESC;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map( |mut desc| {
            let name = unsafe { CStr::from_ptr(desc.SemanticName).to_owned() };
            let index = *self.count.get(&name).unwrap_or(&0);
            desc.SemanticIndex = index;
            self.count.insert(name, index + 1);
            desc
        })
    }
}