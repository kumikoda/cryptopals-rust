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
}
