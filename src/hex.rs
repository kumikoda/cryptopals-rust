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

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn test_decode_hex() {
        let chars = vec![48, 57, 97, 65, 102, 70];
        let bytes = decode(chars);
        let expected = vec![0, 9, 10, 10, 15, 15];
        assert!(bytes == expected)
    }
}
