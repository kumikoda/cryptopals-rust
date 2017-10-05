// Encodes a tightly packed vec<u8> into a base64 string
pub fn encode(bytes: Vec<u8>) -> String {
    // Pad with 0s and keep track of how many
    let mut bytes: Vec<u8> = bytes;
    let to_pad = (3 - bytes.len() % 3) % 3;
    while bytes.len() % 3 != 0 {
        bytes.push(0u8);
    }

    let mut encoded = Vec::new();
    let mut buff = 0u32;

    for (i, &b) in bytes.iter().enumerate() {
        buff <<= 8;
        buff |= b as u32;

        if (i + 1) % 3 == 0 {
            let mut mask = 0b111111000000000000000000;
            for n in 0..4 {
                let value = ((buff & mask) >> 18 - n * 6) as u8;
                encoded.push(encode_one(value));
                mask >>= 6;
            }
        }
    }

    for i in encoded.len() - to_pad..encoded.len() {
        println!("{}", i);
        encoded[i] = b'=';
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

    fn test(input: &str, output: &str) {
        assert_eq!(encode(input.as_bytes().to_vec()), String::from(output));
    }

    #[test]
    fn test_padding() {
        test("M", "TQ==");
        test("Ma", "TWE=");
        test("Man", "TWFu");
        test("any carnal pleasure.", "YW55IGNhcm5hbCBwbGVhc3VyZS4=");
        test("any carnal pleasure", "YW55IGNhcm5hbCBwbGVhc3VyZQ==");
        test("any carnal pleasur", "YW55IGNhcm5hbCBwbGVhc3Vy");
        test("any carnal pleasu", "YW55IGNhcm5hbCBwbGVhc3U=");
        test("any carnal pleas", "YW55IGNhcm5hbCBwbGVhcw==");
    }

    #[test]
    fn test_wikipedia() {
        let input = "\
        Man is distinguished, not only by his reason, but by this singular passion from \
        other animals, which is a lust of the mind, that by a perseverance of delight \
        in the continued and indefatigable generation of knowledge, exceeds the short \
        vehemence of any carnal pleasure.";

        let output = "\
        TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
        IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
        dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
        dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
        ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

        test(input, output);
    }
}
