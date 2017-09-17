extern crate cryptopals;

use cryptopals::conversions::{hex, base64, hex_to_base64};

#[test]
fn set1_problem1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_chars: Vec<u8> = input.as_bytes().to_vec();

    // Decode as hex chars
    let hex_bytes: Vec<u8> = hex::decode(hex_chars);

    // Convert to base64 bytes
    let base64_bytes: Vec<u8> = hex_to_base64(hex_bytes);

    // Encode as base64 chars
    let base64_chars: Vec<u8> = base64::encode(base64_bytes);

    // Bytes to string
    let output: String = String::from_utf8(base64_chars).unwrap();

    let expected = String::from(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );

    assert!(output == expected)
}
