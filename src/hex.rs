// Decodes a hex string into a tightly packed vec<u8>
pub fn decode(s: &str) -> Vec<u8> {
    // Convert to bytes (1 hex char each)
    let mut bytes: Vec<u8> = s.as_bytes().to_vec();

    if bytes.len() % 2 != 0 {
        panic!("invalid hex string: odd length");
    }

    // Pack every 2 chars into 1 u8 buffer
    let mut packed_bytes = Vec::new();
    let mut buff = 0u8;
    for (i, &b) in bytes.iter().enumerate() {
        // decode single character and add to buffer
        match b {
            b'0'...b'9' => buff |= b - b'0',        // 0...9 --> 0...9
            b'a'...b'f' => buff |= b - b'a' + 10,   // a...f --> 10...15
            b'A'...b'F' => buff |= b - b'A' + 10,   // A...F --> 10...15
            _ => panic!("invalid hex char: {}", b),
        }

        // flush every 2 times
        if i % 2 != 0 {
            packed_bytes.push(buff);
            buff = 0u8;
        } else {
            buff <<= 4;
        }
    }

    return packed_bytes;
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn test_decode() {
        let bytes = decode("666f6f626172");
        assert_eq!(bytes, "foobar".as_bytes());
    }
}
