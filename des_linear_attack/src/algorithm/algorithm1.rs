struct LinearCryptanalysis {
    n: usize,  // Number of plaintexts
    t: usize,  // Number of plaintexts where the left side of the equation is 0
    p: f64,    // Probability that the linear approximation holds
}

impl LinearCryptanalysis {
    fn new(n: usize, t: usize, p: f64) -> Self {
        LinearCryptanalysis { n, t, p }
    }

    fn guess_key_bits(&self) -> i32 {
        if self.t > self.n / 2 {
            if self.p > 0.5 {
                0 // Guess that K[k1, k2, ..., kc] equals 0
            } else {
                1 // Guess that K[k1, k2, ..., kc] equals 1
            }
        } else {
            if self.p > 0.5 {
                1 // Guess that K[k1, k2, ..., kc] equals 1
            } else {
                0 // Guess that K[k1, k2, ..., kc] equals 0
            }
        }
    }
}

// fn main() {
//     // Example usage
//     let lc = LinearCryptanalysis::new(1000, 600, 0.75);
//     let guess = lc.guess_key_bits();
//     println!("Guessed key bits: {}", guess);
// }