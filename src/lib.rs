use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Error};

#[proc_macro_derive(SizeOfNoPadding)]
pub fn derive_size_of_no_padding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    if let Err(e) = remove_attrs(&mut input.data) {
        return e.into_compile_error().into();
    }
    input.attrs.clear();
    input.attrs.push(parse_quote!(#[repr(packed)]));
    input.attrs.push(parse_quote!(#[allow(dead_code)]));

    let generics = input.generics.clone();
    let old_ident = input.ident.clone();
    let new_ident = Ident::new(
        &(format!("{}SizeOfNoPadding", input.ident)),
        input.ident.span(),
    );
    input.ident = new_ident.clone();

    quote! {
        #input

        impl #generics #old_ident #generics {
            pub const fn size_of_no_padding() -> usize {
                std::mem::size_of::<#new_ident #generics>()
            }
        }
    }
    .into()
}

fn remove_attrs(data: &mut Data) -> Result<(), Error> {
    match data {
        Data::Struct(s) => {
            match &mut s.fields {
                syn::Fields::Named(n) => n.named.iter_mut().for_each(|f| f.attrs.clear()),
                syn::Fields::Unnamed(u) => u.unnamed.iter_mut().for_each(|f| f.attrs.clear()),
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
