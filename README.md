# Derive macros for BitSparrow in Rust

**[Homepage](http://bitsparrow.io/) -**
**[Cargo](https://crates.io/crates/bitsparrow-derive)**

## Usage

```rust
#[macro_use]
extern crate bitsparrow_derive;
extern crate bitsparrow;

use bitsparrow::*;

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Foo {
    bar: Vec<Bar>,
    baz: String,
    derp: bool,
}

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Bar(u16);

fn main() {
    let foo = Foo {
        bar: vec![Bar(10), Bar(1337)],
        baz: "Hello world".into(),
        derp: true,
    };

    let expect = vec![
        2,                                                      // Vec length
        0x00,0x0A,                                              // |-> 10
        0x05,0x39,                                              // `-> 1337
        11,                                                     // String length
        b'H',b'e',b'l',b'l',b'o',b' ',b'w',b'o',b'r',b'l',b'd', // `-> String data
        1                                                       // bool
    ];

    let buffer = Encoder::encode(&foo);
    let decoded: Foo = Decoder::decode(&buffer).unwrap();

    assert_eq!(buffer, expect);
    assert_eq!(decoded, foo);
}
```
