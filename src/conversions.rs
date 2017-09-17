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
    #[test]
    fn test_hex_to_base64() {}
}
