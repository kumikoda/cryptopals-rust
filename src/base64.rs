// Encodes a tightly packed vec<u8> into a base64 string
pub fn encode(bytes: Vec<u8>) -> String {
    let mut encoded = Vec::new();
    let mut buff = 0u32;

    let mut to_flush = bytes.len() % 3 + 1;
    let mut to_pad = 4 - to_flush;

    let mut mask = 0b111111000000000000000000;

    for (i, &b) in bytes.iter().enumerate() {
        buff <<= 8;
        buff |= b as u32;

        if (i + 1) % 3 == 0 {
            mask = 0b111111000000000000000000;
            for n in 0..4 {
                let value = ((buff & mask) >> 18 - n * 6) as u8;
                encoded.push(encode_one(value));
                mask >>= 6;
            }
        }
    }

    println!("to_flush:{}", to_flush);
    println!("to_pad:{}", to_pad);
    for n in 0..to_flush {
        println!("buff: {:#b}", buff);
        println!("mask: {:#b}", mask);
        println!("after: {:#b}", buff & mask);
        let value = ((buff & mask) >> n * 6) as u8;
        println!("{}", value);
        encoded.push(encode_one(value));
        mask >>= 6;
    }

    for i in 0..to_pad {
        encoded.push(b'=');
    }

    String::from_utf8(encoded).unwrap()
}

fn encode_one(b: u8) -> u8 {
    match b {
        0...25 => b + b'A', // A...Z
        26...51 => b - 26 + b'a', // a...z
        52...61 => b - 52 + b'0', // 0...9
        62 => b'+', // special symbol
        63 => b'/', // special symbol
        _ => panic!("invalid value: {}", b),
    }
}

#[cfg(test)]
mod tests {
    use super::encode;
    #[test]
    fn test_padding() {
        assert_eq!(encode([b'M'].to_vec()), String::from("TQ=="));
        // assert_eq!(encode([b'M', b'a'].to_vec()), String::from("TWE="));
        //assert_eq!(encode([b'M', b'a', b'n'].to_vec()), String::from("TWFu"));
    }
    #[test]
    fn test_wikipedia() {
        let bytes = "\
        Man is distinguished, not only by his reason, but by this singular passion from\
        other animals, which is a lust of the mind, that by a perseverance of delight\
        in the continued and indefatigable generation of knowledge, exceeds the short\
        vehemence of any carnal pleasure."
            .as_bytes()
            .to_vec();

        let expected = String::from(
            "\
        TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
        IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
        dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
        dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
        ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=",
        );

        //let encoded = encode(bytes);
        //println!("{}", encoded);
        //     assert!(encoded == expected);
    }
}
