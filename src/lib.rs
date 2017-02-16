//! # `#[derive]` macros for BitSparrow in Rust
//!
//! **[Homepage](http://!bitsparrow.io/) -**
//! **[Cargo](https://!crates.io/crates/bitsparrow-derive)**
//!
//! ## Usage
//!
//! ```
//! #[macro_use]
//! extern crate bitsparrow_derive;
//! extern crate bitsparrow;
//!
//! use bitsparrow::*;
//!
//! #[derive(BitEncode, BitDecode, PartialEq, Debug)]
//! struct Foo {
//!     bar: Vec<Bar>,
//!     baz: String,
//!     derp: bool,
//! }
//!
//! #[derive(BitEncode, BitDecode, PartialEq, Debug)]
//! struct Bar(u16);
//!
//! fn main() {
//!     let foo = Foo {
//!         bar: vec![Bar(10), Bar(1337)],
//!         baz: "Hello world".into(),
//!         derp: true,
//!     };
//!
//!     let expect = vec![
//!         2,                                                      //! Vec length
//!         0x00,0x0A,                                              //! |-> 10
//!         0x05,0x39,                                              //! `-> 1337
//!         11,                                                     //! String length
//!         b'H',b'e',b'l',b'l',b'o',b' ',b'w',b'o',b'r',b'l',b'd', //! `-> String data
//!         1                                                       //! bool
//!     ];
//!
//!     let buffer = Encoder::encode(&foo);
//!     let decoded: Foo = Decoder::decode(&buffer).unwrap();
//!
//!     assert_eq!(buffer, expect);
//!     assert_eq!(decoded, foo);
//! }
//! ```

// The `quote!` macro requires deep recursion.
#![recursion_limit = "192"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

mod encode;
mod decode;

use proc_macro::TokenStream;
use encode::{encode_struct, encode_enum};
use decode::{decode_struct, decode_enum};
use syn::Body;

#[proc_macro_derive(BitEncode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();

    let ident = input.ident;

    let (size_hint, body) = match input.body {
        Body::Struct(body) => encode_struct(body),
        Body::Enum(variants) => encode_enum(&ident, variants),
    };

    let tokens = quote! {
        impl BitEncode for #ident {
            fn encode(&self, e: &mut Encoder) {
                #body
            }

            #[inline]
            fn size_hint() -> usize {
                #size_hint
            }
        }

        impl<'a> BitEncode for &'a #ident {
            #[inline]
            fn encode(&self, e: &mut Encoder) {
                BitEncode::encode(*self, e)
            }

            #[inline]
            fn size_hint() -> usize {
                #size_hint
            }
        }
    };

    tokens.parse().unwrap()
}

#[proc_macro_derive(BitDecode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();

    let ident = input.ident;

    let body = match input.body {
        Body::Struct(body) => decode_struct(&ident, body),
        Body::Enum(variants) => decode_enum(&ident, variants),
    };

    let tokens = quote! {
        impl BitDecode for #ident {
            fn decode(d: &mut Decoder) -> Result<Self, Error> {
                Ok(#body)
            }
        }
    };

    tokens.parse().unwrap()
}
