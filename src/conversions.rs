pub fn hex_to_base64(hex_bytes: Vec<u8>) -> Vec<u8> {
    // pad with 0's until divisible by 3
    let mut hex_bytes = hex_bytes;
    while hex_bytes.len() % 3 != 0 {
        hex_bytes.push(0);
    }

    // vector to hold final output
    let mut b64_bytes: Vec<u8> = vec![];

    // container for 3*4 bits
    let mut buff = 0u16;

    // for every three hex (4bits) we create two base64 (6 bits)
    for (i, &hex) in hex_bytes.iter().enumerate() {
        // shift hex to buffer from the right
        buff <<= 4;
        buff += hex as u16;

        // once we did it thre times...
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
    use super::hex_to_base64;

    #[test]
    fn test_hex_to_base64() {
        // write a good test!
    }
}
