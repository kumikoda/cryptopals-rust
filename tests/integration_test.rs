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
