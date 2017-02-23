extern crate bitsparrow;
#[macro_use]
extern crate bitsparrow_derive;

use bitsparrow::*;

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Foo<'a> {
    bar: Vec<Bar>,
    baz: &'a str,
    derp: bool,
}

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct OwnedFoo {
    bar: Vec<Bar>,
    baz: String,
    derp: bool,
}

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
struct Bar(u16);

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
enum Doge<'a> {
    Moon {
        is: &'a str,
        of: String,
    },
    To(Vec<u16>),
    The,
}

#[derive(BitEncode, BitDecode, PartialEq, Debug)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[test]
fn structs() {
    let foo = Foo {
        bar: vec![Bar(10), Bar(1337)],
        baz: "Hello world",
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

#[test]
fn owned_structs() {
    let foo = OwnedFoo {
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
    let decoded: OwnedFoo = Decoder::decode(&buffer).unwrap();

    assert_eq!(buffer, expect);
    assert_eq!(decoded, foo);
}

#[test]
fn enums() {
    let doges = (
        Doge::To(vec![1,2,3]),
        Doge::The,
        Doge::Moon { is: "made", of: "cheese".into() }
    );

    let expect = vec![
        0x01,0x03,0x00,0x01,0x00,0x02,0x00,0x03,
        0x02,
        0x00,0x04,b'm',b'a',b'd',b'e',0x06,b'c',b'h',b'e',b'e',b's',b'e'
    ];

    let buffer = Encoder::encode(&doges);
    let decoded: (Doge, Doge, Doge) = Decoder::decode(&buffer).unwrap();

    assert_eq!(buffer, expect);
    assert_eq!(decoded, doges);
}

#[test]
fn simple_enum() {
    use Letter::*;

    let hello = (H,E,L,L,O,W,O,R,L,D);

    let buffer = Encoder::encode(&hello);

    let decoded: (Letter,Letter,Letter,Letter,Letter,Letter,Letter,Letter,Letter,Letter) = Decoder::decode(&buffer).unwrap();

    assert_eq!(buffer.len(), 10);
    assert_eq!(hello, decoded);
}
