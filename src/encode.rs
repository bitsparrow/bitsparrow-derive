use quote::Tokens;
use syn::{Ident, VariantData, Variant};

fn encode_ident(ident: Ident) -> Tokens {
    quote! { ::bitsparrow::BitEncode::encode(&self.#ident, e); }
}

pub fn encode_struct(mut body: VariantData) -> (usize, Tokens) {
    let fields = match body {
        VariantData::Struct(ref mut body) => {
            body.iter_mut()
                .map(|field| field.ident.take().unwrap())
                .map(encode_ident)
                .collect()
        },
        VariantData::Tuple(ref body) => {
            body.iter()
                .enumerate()
                .map(|(i, _)| i.to_string().into())
                .map(encode_ident)
                .collect()
        },
        VariantData::Unit => Vec::new(),
    };

    (8 * fields.len(), quote! { #( #fields )* })
}

pub fn encode_enum(ident: &Ident, variants: Vec<Variant>) -> (usize, Tokens) {
    let matches = variants.iter().enumerate().map(|(index, variant)| {
        let varident = variant.ident.clone();

        match variant.data {
            VariantData::Struct(ref body) => {
                let idents: Vec<Ident> = body
                    .iter()
                    .map(|field| field.ident.clone().unwrap())
                    .collect();

                let refs = &idents;

                quote! {
                    #ident::#varident {#( ref #refs ),*} => {
                        ::bitsparrow::BitEncode::encode(&#index, e);
                        #( ::bitsparrow::BitEncode::encode(#refs, e); )*
                    },
                }
            },
            VariantData::Tuple(ref body) => {
                let idents: Vec<Ident> = body
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::from(format!("ref{}", i)))
                    .collect();

                let refs = &idents;

                quote! {
                    #ident::#varident(#( ref #refs ),*) => {
                        ::bitsparrow::BitEncode::encode(&#index, e);
                        #( ::bitsparrow::BitEncode::encode(#refs, e); )*
                    },
                }
            },
            VariantData::Unit => quote! {
                #ident::#varident => return ::bitsparrow::BitEncode::encode(&#index, e),
            },
        }
    });

    (1, quote! { match *self { #( #matches )* }; })
}
