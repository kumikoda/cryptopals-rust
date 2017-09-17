pub mod hex {
    pub fn decode(bytes: Vec<u8>) -> Vec<u8> {
        bytes.into_iter().map(decode_one).collect()
    }

    fn decode_one(b: u8) -> u8 {
        match b {
            48...57 => b - 48, // 0...9 --> 0...9
            97...102 => b - 87, // a...f --> 10...15
            65...90 => b - 55, // A...F --> 10...15
            _ => panic!("invalid hex: {}", b),
        }
    }
}

pub mod base64 {
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
}

pub fn hex_to_base64(hex_bytes: Vec<u8>) -> Vec<u8> {
    // strategy: for every three hex (4bits) we create two base64 (6 bits)

    // pad with 0's until divisible by 3
    if hex_bytes.len() % 3 != 0 {
        println!("HAHA");
    }

    // vector to hold final output
    let mut b64_bytes: Vec<u8> = vec![];

    // container for 3*4 bits
    let mut buff = 0u16;

    for (i, &hex) in hex_bytes.iter().enumerate() {
        // shift hex (4 bits) in from the right
        buff <<= 4;
        buff += hex as u16;

        // once we did it three times...
        if (i + 1) % 3 == 0 {
            // take most significant six as first base64
            b64_bytes.push(((buff & 0b111111000000) >> 6) as u8);

            // take least significant six bits as second base64
            b64_bytes.push((buff & 0b000000111111) as u8);

            // reset the buffer
            buff = 0u16;
        }
    }

    return b64_bytes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_hex() {
        let chars = vec![48, 57, 97, 65, 102, 70];
        let bytes = hex::decode(chars);
        let expected = vec![0, 9, 10, 10, 15, 15];
        assert!(bytes == expected)
    }

    #[test]
    fn test_encode_base64() {
        let bytes = vec![0, 25, 26, 51, 52, 61, 62, 63];
        let chars = base64::encode(bytes);
        let expected = "AZaz09+/".as_bytes();
        assert!(chars == expected)
    }

    #[test]
    fn test_hex_to_base64() {}
}
