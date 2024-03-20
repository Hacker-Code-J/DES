pub fn dot(alpha: u8, beta: u8) -> u8 {
    (0..8).fold(0, |acc, i| acc ^ ((alpha >> i) & (beta >> i) & 1))
    // (alpha & beta).count_ones() as u8 % 2
}