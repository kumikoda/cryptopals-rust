pub fn fixed_xor(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Vec<u8> {
    let mut xored = Vec::new();

    if bytes1.len() != bytes2.len() {
        panic!("invalid inputs: s1 and s2 must be same length");
    }

    for i in 0..bytes1.len() {
        xored.push(bytes1[i] ^ bytes2[i]);
    }

    return xored;
}

pub fn single_xor(bytes: Vec<u8>, b: u8) -> Vec<u8> {
    let mut xored = Vec::new();

    for i in 0..bytes.len() {
        xored.push(bytes[i] ^ b);
    }

    return xored;
}
