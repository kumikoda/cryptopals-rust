fn log(s: &str, x: u32) {
    println!("{}: {:<024b}", s, x);
}

// Character encodings for base64
static B64: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789+/";

fn to_b64(x: u32) -> char {
    B64[x as usize] as char
}

fn to_base64_chars(buff: u32) -> Vec<char> {
    let a = (buff & 0b111111000000000000000000) >> 18;
    let b = (buff & 0b000000111111000000000000) >> 12;
    let c = (buff & 0b000000000000111111000000) >> 6;
    let d = (buff & 0b000000000000000000111111) >> 0;
    vec![to_b64(a), to_b64(b), to_b64(c), to_b64(d)]
}

// Hex strings are only 4 bits
fn as_hex(c: u8) -> u8 {
    match c {
        48...57 => c - 48, // numbers
        97...102 => c - 87, // characters
        _ => panic!("invalid hex"),
    }
}

fn hex_to_base64(s: &str) -> String {
    // Overall strategy, 3 bytes --> 32 bits --> base 64
    let mut chars = Vec::new();

    // Convert the slice of string to a slice of bytes
    let bytes: &[u8] = s.as_bytes();

    // Add every 6 bytes to a 32bit buffer
    let mut buff: u32 = 0;
    for (i, &item) in bytes.iter().enumerate() {
        // first decode as hex
        let decoded = as_hex(item);
        // upsize it to u32 so we can shift without dropping
        let block = decoded as u32;
        // then shift it to the left!
        let mask = block << 4 * (5 - i % 6);
        // finally add it to the buffer
        buff = buff | mask;
        // if we added 3 bytes, save as 4 base64 chars and flush
        if (i + 1) % 6 == 0 && i != 0 {
            chars.extend(to_base64_chars(buff));
            buff = 0;
        }
    }

    // return as String
    chars.into_iter().collect()
}

fn main() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let o = hex_to_base64(s);
    println!("{}", o)
}
