// Scores a string according to letter frequency
pub fn score(s: &str) -> f64 {
    // from https://en.wikipedia.org/wiki/Letter_frequency
    // plus two hand tuned values for special characters and spaces :)
    let expected = vec![
        8.167,
        1.492,
        2.782,
        4.253,
        12.702,
        2.228,
        2.015,
        6.094,
        6.966,
        0.153,
        0.772,
        4.025,
        2.406,
        6.749,
        7.507,
        1.929,
        0.095,
        5.987,
        6.327,
        9.056,
        2.758,
        0.978,
        2.360,
        0.150,
        1.974,
        0.074,
        5.0, // expect some spaces
        0.0, // and no special characters
    ];

    let mut freqs = vec![0.0; 28];
    for (i, &c) in s.as_bytes().iter().enumerate() {
        match c {
            b'a'...b'z' => freqs[(c - b'a') as usize] = freqs[(c - b'a') as usize] + 1.0,
            b'A'...b'Z' => freqs[(c - b'A') as usize] = freqs[(c - b'A') as usize] + 1.0,
            b' ' => freqs[26] = freqs[26] + 1.0,
            _ => freqs[27] = freqs[27] + 1.0,
        }
    }

    return cos_sim(freqs, expected);
}

// Returns the cosine similarity of two equal length vectors
pub fn cos_sim(A: Vec<f64>, B: Vec<f64>) -> f64 {
    if A.len() != B.len() {
        panic!("a and b must be same length");
    }

    if A.len() == 0 {
        panic!("must have non-zero length");
    }

    let mut sum_of_products = 0f64;
    let mut sum_of_a_squares = 0f64;
    let mut sum_of_b_squares = 0f64;

    for i in 0..A.len() {
        let a = A[i];
        let b = B[i];
        sum_of_products = sum_of_products + a * b;
        sum_of_a_squares = sum_of_a_squares + a * a;
        sum_of_b_squares = sum_of_b_squares + b * b;
    }

    return sum_of_products / (sum_of_a_squares.sqrt() * sum_of_b_squares.sqrt());
}

#[cfg(test)]
mod test {
    use super::{cos_sim, score};

    #[test]
    fn test_cos_sim_1() {
        let vec1 = vec![0.1, 0.2, 0.3];
        let vec2 = vec![0.1, 0.2, 0.3];

        let sim = cos_sim(vec1, vec2);

        assert_eq!(sim, 1.0);
    }

    #[test]
    fn test_cos_sim_0() {
        let vec1 = vec![0.1, 0.0, 0.0];
        let vec2 = vec![0.0, 0.1, 0.0];

        let sim = cos_sim(vec1, vec2);

        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_cos_sim_neg_1() {
        let vec1 = vec![0.1, 0.2, 0.3];
        let vec2 = vec![-0.1, -0.2, -0.3];

        let sim = cos_sim(vec1, vec2);

        assert_eq!(sim, -1.0);
    }

    #[test]
    fn test_score_sentence() {
        let s1 = "Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid";
        let s2 = "xxxxxxxxxxxxxxxxxxxxxxxxxyyyyyyyyyyyyyyyyyyyyzzzzzzzzzzzzzzz";
        assert!(score(s1) > score(s2));
    }
}
