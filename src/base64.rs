pub fn encode(bytes: Vec<u8>) -> Vec<u8> {
    bytes.into_iter().map(encode_one).collect()
}

fn encode_one(b: u8) -> u8 {
    match b {
        0...25 => b + 65, // A...Z
        26...51 => b + 71, // a...z
        52...61 => b - 4, // 0...9
        62 => 43, // `+`
        63 => 47, // `/`
        _ => panic!("cannot b64 encode {}", b),
    }
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn test_encode_base64() {
        let bytes = vec![0, 25, 26, 51, 52, 61, 62, 63];
        let chars = encode(bytes);
        let expected = "AZaz09+/".as_bytes();
        assert!(chars == expected)
    }

    #[test]
    fn test_wikipedia() {
        // - move the encoding logic from conversions to here
        // - add logic to stuff two hex into one u8 inside conversions
        // - need to make the API encode u8s properly


        let bytes = "\
        Man is distinguished, not only by his reason, but by this singular passion from\
        other animals, which is a lust of the mind, that by a perseverance of delight\
        in the continued and indefatigable generation of knowledge, exceeds the short\
        vehemence of any carnal pleasure."
            .as_bytes()
            .to_vec();

        let expected = "\
        TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
        IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
        dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
        dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
        ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4="
            .as_bytes()
            .to_vec();

        let encoded = encode(bytes);
        println!("{:?}", encoded);
        assert!(encoded == expected);

    }
}
