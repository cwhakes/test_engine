macro_rules! vertex_generate {
    ($vertex: ident, $inner: ty, $name: expr, $format: expr) => {
        #[repr(C)]
        #[derive(Clone, Debug, Default)]
        pub struct $vertex($inner);

        impl Vertex for $vertex {
            fn desc(offset: usize) -> Box<dyn Iterator<Item = d3d11::D3D11_INPUT_ELEMENT_DESC>> {
                let semantic_name = CStr::from_bytes_with_nul($name).unwrap();

                let desc = d3d11::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: semantic_name.as_ptr(),
                    SemanticIndex: 0,
                    Format: $format,
                    InputSlot: 0,
                    AlignedByteOffset: offset as u32,
                    InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                };

                Box::new(Some(desc).into_iter())
            }
        }

        impl<T: Into<$inner>> std::convert::From<T> for $vertex {
            fn from(vector: T) -> Self {
                $vertex(vector.into())
            }
        }

        impl std::ops::Deref for $vertex {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $vertex {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
