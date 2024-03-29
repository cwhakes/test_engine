extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Field, Fields, GenericParam, Generics,
};

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let chain = desc_chain(&input.data);

    let expanded = quote! {
        impl #impl_generics engine::graphics::vertex::Vertex for #name #ty_generics #where_clause {
            fn desc(offset: usize) -> Box<dyn Iterator<Item = engine::graphics::vertex::D3D11_INPUT_ELEMENT_DESC>> {
                #chain
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(engine::graphics::vertex::Vertex));
        }
    }
    generics
}

fn desc_chain(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => desc_chain_inner(fields.named.iter()),
            Fields::Unnamed(ref fields) => desc_chain_inner(fields.unnamed.iter()),
            Fields::Unit => {
                quote! {
                    Box::new(None.into_iter())
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn desc_chain_inner<'a>(iter: impl Iterator<Item = &'a Field>) -> TokenStream {
    let recurse = iter.map(|f| {
        let ty = &f.ty;
        quote_spanned! {f.span()=>
            let iter = iter.chain(#ty::desc(offset));
            #[allow(unused_variables)]
            let offset = offset + std::mem::size_of::<#ty>();
        }
    });
    quote! {
        let iter = None.into_iter();
        #(#recurse)*
        Box::new(iter)
    }
}
