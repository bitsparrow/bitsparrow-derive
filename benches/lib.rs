#![feature(test)]

extern crate test;
extern crate bitsparrow;
#[macro_use]
extern crate bitsparrow_derive;

use bitsparrow::*;

use test::Bencher;

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Foo {
    bar: String,
    baz: u64,
    derp: bool,
}

#[bench]
fn encode_derived_struct(b: &mut Bencher) {
    let foo = Foo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    b.iter(|| {
        Encoder::encode(&foo)
    })
}

#[bench]
fn decode_derived_struct(b: &mut Bencher) {
    let foo = Foo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    let buffer = Encoder::encode(&foo);

    b.iter(|| {
        let _foo: Foo = Decoder::decode(&buffer).unwrap();
    })
}
