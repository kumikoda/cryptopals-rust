extern crate cryptopals;

use cryptopals::{hex, base64, conversions};

#[test]
fn set1_problem1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    // Decode as hex
    let bytes: Vec<u8> = hex::decode(input);

    // Encode as base64
    let output: String = base64::encode(bytes);

    let expected = String::from(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );

    assert_eq!(output, expected);
}

#[test]
fn set1_problem2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";

    let output = hex::fixed_xor(input1, input2);

    let expected = String::from("746865206b696420646f6e277420706c6179");
}
