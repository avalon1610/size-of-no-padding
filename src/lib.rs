use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Error, Meta};

#[proc_macro_derive(SizeOfNoPadding)]
pub fn derive_size_of_no_padding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    if let Data::Enum(e) = input.data {
        return Error::new(e.enum_token.span, "SizeOfNoPadding not work for enum")
            .into_compile_error()
            .into();
    }

    let mut found = false;
    input.attrs.iter_mut().for_each(|attr| {
        if let Meta::List(ml) = &mut attr.meta {
            for seg in &mut ml.path.segments {
                if seg.ident == "repr" {
                    ml.tokens = quote!(packed);
                    found = true;
                }
            }
        }
    });

    if !found {
        input.attrs.push(parse_quote!(#[repr(packed)]));
    }

    let generics = input.generics.clone();
    let old_ident = input.ident.clone();
    let new_ident = Ident::new(
        &(format!("{}_SizeOfNoPadding", input.ident)),
        input.ident.span(),
    );
    input.ident = new_ident.clone();

    quote! {
        #input

        impl #generics #old_ident #generics {
            const fn size_of_no_padding() -> usize {
                std::mem::size_of::<#new_ident #generics>()
            }
        }
    }
    .into()
}
