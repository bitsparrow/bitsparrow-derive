use quote::Tokens;
use syn::{Ident, VariantData, Variant};

pub fn decode_struct(ident: &Ident, body: VariantData) -> Tokens {
    match body {
        VariantData::Struct(ref body) => {
            let fields = body
                .iter()
                .map(|field| &field.ident)
                .map(|ident| quote! { #ident: ::bitsparrow::BitDecode::decode(d)?, });

            quote! { #ident{ #( #fields )* } }
        },
        VariantData::Tuple(ref body) => {
            let fields = body.iter().map(|_| quote! { ::bitsparrow::BitDecode::decode(d)? });

            quote! { #ident( #( #fields )* ) }
        },
        VariantData::Unit => quote! { #ident }
    }
}

pub fn decode_enum(ident: &Ident, variants: Vec<Variant>) -> Tokens {
    let matches = variants.into_iter().enumerate().map(|(index, variant)| {
        let varident = variant.ident.clone();
        let varstruct = decode_struct(&varident, variant.data);

        quote! { #index => #ident::#varstruct, }
    });

    quote! {
        match ::bitsparrow::BitDecode::decode(d)? {
            #( #matches )*
            _ => return Err(::bitsparrow::Error::InvalidData)
        }
    }
}
