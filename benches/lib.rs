#![feature(test)]

extern crate test;
extern crate bitsparrow;

#[macro_use]
extern crate bitsparrow_derive;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate bincode;

use bitsparrow::{Encoder, Decoder};
use bincode::{serialize, deserialize};
use bincode::SizeLimit::Infinite;

use test::Bencher;

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Foo<'a> {
    bar: &'a str,
    baz: u64,
    derp: bool,
}

#[derive(Serialize, Deserialize, BitEncode, BitDecode, PartialEq, Debug)]
struct OwnedFoo {
    bar: String,
    baz: u64,
    derp: bool,
}

#[bench]
fn borrow_encode_derived_struct(b: &mut Bencher) {
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
fn borrow_decode_derived_struct(b: &mut Bencher) {
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
fn owned_encode_derived_struct(b: &mut Bencher) {
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
fn owned_decode_derived_struct(b: &mut Bencher) {
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

#[bench]
fn bincode_encode_derived_owned_struct(b: &mut Bencher) {
    let foo = OwnedFoo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    b.iter(|| {
        serialize(&foo, Infinite).unwrap()
    })
}

#[bench]
fn bincode_decode_derived_owned_struct(b: &mut Bencher) {
    let foo = OwnedFoo {
        bar: "hello".into(),
        baz: 1337u64,
        derp: true,
    };

    let buffer = serialize(&foo, Infinite).unwrap();

    b.iter(|| {
        let _owned_foo: OwnedFoo = deserialize(&buffer).unwrap();
    })
}
