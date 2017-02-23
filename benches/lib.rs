#![feature(test)]

extern crate test;
extern crate bitsparrow;
#[macro_use]
extern crate bitsparrow_derive;

use bitsparrow::*;

use test::Bencher;

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Foo<'a> {
    bar: &'a str,
    baz: u64,
    derp: bool,
}

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct OwnedFoo {
    bar: String,
    baz: u64,
    derp: bool,
}

#[bench]
fn encode_derived_struct(b: &mut Bencher) {
    let foo = Foo {
        bar: "hello",
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
        bar: "hello",
        baz: 1337u64,
        derp: true,
    };

    let buffer = Encoder::encode(&foo);

    b.iter(|| {
        let _foo: Foo = Decoder::decode(&buffer).unwrap();
    })
}

#[bench]
fn encode_derived_owned_struct(b: &mut Bencher) {
    let foo = OwnedFoo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    b.iter(|| {
        Encoder::encode(&foo)
    })
}

#[bench]
fn decode_derived_owned_struct(b: &mut Bencher) {
    let foo = OwnedFoo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    let buffer = Encoder::encode(&foo);

    b.iter(|| {
        let _owned_foo: OwnedFoo = Decoder::decode(&buffer).unwrap();
    })
}
