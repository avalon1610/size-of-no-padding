use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Error, Fields, Index};

/// Calculate the size for every field, add up all the size. call `size_of_no_padding_any(&self)`
/// will give the size of whole struct without padding in runtime.
#[proc_macro_derive(SizeOfNoPaddingAny)]
pub fn derive_size_if_no_padding_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let generics = &input.generics;
    let generic_types = &input
        .generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Type(t) => Some(&t.ident),
            syn::GenericParam::Const(c) => Some(&c.ident),
            _ => None,
        })
        .collect::<Vec<_>>();
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(n) => n.named.iter(),
            Fields::Unnamed(u) => u.unnamed.iter(),
            Fields::Unit => {
                return Error::new(
                    Span::call_site(),
                    "SizeOfNoPaddingAny not applicable for this type",
                )
                .into_compile_error()
                .into();
            }
        },
        Data::Union(u) => u.fields.named.iter(),
        Data::Enum(e) => {
            return Error::new(e.enum_token.span, "SizeOfNoPaddingAny not work for enum")
                .into_compile_error()
                .into();
        }
    }
    .enumerate()
    .map(|(i, f)| {
        f.ident.as_ref().map(|n| quote! { #n }).unwrap_or({
            let i = Index::from(i);
            quote! { #i }
        })
    })
    .collect::<Vec<_>>();

    quote! {
        impl #generics size_of_no_padding::SizeOfAny for #struct_name <#(#generic_types),*> {
            fn size_of_no_padding_any(&self) -> usize {
                let mut size = 0usize;

                #(
                    size += self.#fields.size_of_no_padding_any();
                )*

                size
            }
        }
    }
    .into()
}

/// Create a shadow struct which as same as original struct except it's marked `#[repr(packed)]`.
/// call `size_of_no_padding()` method on struct will invoke `std::mem::size_of()` function on shadow one in compile time.
#[proc_macro_derive(SizeOfNoPadding)]
pub fn derive_size_of_no_padding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    if let Err(e) = remove_attrs(&mut input.data) {
        return e.into_compile_error().into();
    }
    input.attrs.clear();
    input.attrs.push(parse_quote!(#[derive(Copy, Clone)]));
    input.attrs.push(parse_quote!(#[repr(C, packed)]));
    input.attrs.push(parse_quote!(#[allow(dead_code)]));

    let generics = input.generics.clone();
    let generic_types = &input
        .generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Type(t) => Some(&t.ident),
            syn::GenericParam::Const(c) => Some(&c.ident),
            _ => None,
        })
        .collect::<Vec<_>>();
    let old_ident = input.ident.clone();
    let new_ident = Ident::new(
        &(format!("{}SizeOfNoPadding", input.ident)),
        input.ident.span(),
    );
    input.ident = new_ident.clone();

    quote! {
        #input

        impl #generics size_of_no_padding::SizeOfAny for #old_ident <#(#generic_types),*> {
            fn size_of_no_padding_any(&self) -> usize {
                use ::size_of_no_padding::SizeOf;
                Self::size_of_no_padding()
            }
        }

        impl #generics size_of_no_padding::SizeOf for #old_ident <#(#generic_types),*> {
            fn size_of_no_padding() -> usize {
                std::mem::size_of::<#new_ident <#(#generic_types),*>>()
            }
        }
    }
    .into()
}

fn remove_attrs(data: &mut Data) -> Result<(), Error> {
    match data {
        Data::Struct(s) => {
            match &mut s.fields {
                Fields::Named(n) => n.named.iter_mut().for_each(|f| f.attrs.clear()),
                Fields::Unnamed(u) => u.unnamed.iter_mut().for_each(|f| f.attrs.clear()),
                _ => {}
            }
            Ok(())
        }
        Data::Union(u) => {
            u.fields.named.iter_mut().for_each(|f| f.attrs.clear());
            Ok(())
        }
        Data::Enum(e) => Err(Error::new(
            e.enum_token.span,
            "SizeOfNoPadding not work for enum",
        )),
    }
}
