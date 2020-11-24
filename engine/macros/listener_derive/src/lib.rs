extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Meta, NestedMeta};

#[proc_macro_derive(Listener, attributes(listener))]
pub fn derive_listener(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_string = name.to_string();
    let on_key_down = make_method(&input.data, on_key_down);
    let on_key_up = make_method(&input.data, on_key_up);
    let on_mouse_move = make_method(&input.data, on_mouse_move);
    let on_left_mouse_down = make_method(&input.data, on_left_mouse_down);
    let on_right_mouse_down = make_method(&input.data, on_right_mouse_down);
    let on_left_mouse_up = make_method(&input.data, on_left_mouse_up);
    let on_right_mouse_up = make_method(&input.data, on_right_mouse_up);

    let parent = find_parent_fns(&input.attrs);
    let on_key_up_parent = parent
        .get("on_key_up")
        .map(|stream| {
            quote! { self.#stream(key); }
        })
        .unwrap_or_default();

    let expanded = quote! {
        impl engine::input::Listener for #name {
            fn name(&self) -> String {#name_string.to_string()}
            fn on_key_down(&mut self, key: usize) { #on_key_down }
            fn on_key_up(&mut self, key: usize) {
                #on_key_up_parent
                #on_key_up
            }

            fn on_mouse_move(&mut self, pos: Point) { #on_mouse_move }
            fn on_left_mouse_down(&mut self) { #on_left_mouse_down }
            fn on_right_mouse_down(&mut self) { #on_right_mouse_down }
            fn on_left_mouse_up(&mut self) { #on_left_mouse_up }
            fn on_right_mouse_up(&mut self) { #on_right_mouse_up }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn on_key_down(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_key_down(&mut self.#name, key);
    }
}

fn on_key_up(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_key_up(&mut self.#name, key);
    }
}

fn on_mouse_move(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_mouse_move(&mut self.#name, pos);
    }
}

fn on_left_mouse_down(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_left_mouse_down(&mut self.#name);
    }
}

fn on_right_mouse_down(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_right_mouse_down(&mut self.#name);
    }
}

fn on_left_mouse_up(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_left_mouse_up(&mut self.#name);
    }
}

fn on_right_mouse_up(f: Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        engine::input::Listener::on_right_mouse_up(&mut self.#name);
    }
}

fn make_method<F>(data: &Data, function: F) -> TokenStream
where
    F: Fn(Field) -> TokenStream,
{
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => make_method_inner(fields.named.iter(), function),
            Fields::Unnamed(ref fields) => make_method_inner(fields.unnamed.iter(), function),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn make_method_inner<'a, F>(iter: impl Iterator<Item = &'a Field>, function: F) -> TokenStream
where
    F: Fn(Field) -> TokenStream,
{
    let recurse = iter
        .cloned()
        .filter(|f| f.attrs.iter().any(|a| a.path.is_ident("listener")))
        .map(function);
    quote! {
        #(#recurse)*
    }
}

fn find_parent_fns<'a>(
    attributes: impl IntoIterator<Item = &'a Attribute>,
) -> HashMap<String, TokenStream> {
    let mut map = HashMap::new();
    for attribute in attributes.into_iter() {
        if attribute.path.is_ident("listener") {
            if let Meta::List(list) = attribute.parse_meta().unwrap() {
                for nested_meta in list.nested.iter() {
                    match nested_meta {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            map.insert(
                                path.segments.last().unwrap().ident.to_string(),
                                path.to_token_stream(),
                            );
                        }
                        _ => unimplemented!(),
                    }
                }
            } else {
                panic!("Incorrect listener calling convention")
            }
        }
    }
    map
}
