extern crate cryptopals;

use cryptopals::{hex, base64, crypto, english};

#[test]
fn set1_problem1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let bytes: Vec<u8> = hex::decode(input);

    let output: String = base64::encode(bytes);

    let expected = String::from(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );

    assert_eq!(output, expected);
}

#[test]
fn set1_problem2() {
    let bytes1 = hex::decode("1c0111001f010100061a024b53535009181c");
    let bytes2 = hex::decode("686974207468652062756c6c277320657965");

    let output = crypto::fixed_xor(bytes1, bytes2);
    let encoded = hex::encode(output);

    let expected = String::from("746865206b696420646f6e277420706c6179");

    assert_eq!(encoded, expected);
}

#[test]
fn set1_problem3() {
    let mut max_score = 0.0;
    let mut answer = String::from("");
    for i in b'A'..b'Z' {
        let input = hex::decode(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        );
        let bytes = crypto::single_xor(input, i);
        let decoded = String::from_utf8(bytes).unwrap();
        let score = english::score(decoded.as_str());
        println!("{}, {}", decoded, score);
        if score > max_score {
            answer = decoded;
            max_score = score;
        }
    }

    assert_eq!("Cooking MC's like a pound of bacon", answer);
}
