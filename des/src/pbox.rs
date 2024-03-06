// Define the module for P-box (permutation box) used in DES.

// Fixed permutation table for DES P-box according to the DES standard.
const P: [usize; 32] = [
    16,  7, 20, 21,
    29, 12, 28, 17,
     1, 15, 23, 26,
     5, 18, 31, 10,
     2,  8, 24, 14,
    32, 27,  3,  9,
    19, 13, 30,  6,
    22, 11,  4, 25,
];

/// Performs the DES permutation on a 32-bit block.
///
/// # Arguments
///
/// * `input` - The 32-bit input block to be permuted.
///
/// # Returns
///
/// * The 32-bit output block after permutation.
pub fn des_pbox(input: u32) -> u32 {
    let mut output = 0;
    for &i in &P {
        output <<= 1;
        output ^= (input >> (32 - i)) & 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_des_pbox() {
        // Example test case: Using a pattern where input equals output to check if permutation works.
        // Note: Replace '0x12345678' with actual test values based on DES specifications
        let input = 0x12345678; // Example input, replace with actual
        let expected = 0x87654321; // Expected output, replace with actual
        let result = des_pbox(input);
        assert_eq!(result, expected, "P-box permutation failed.");
    }
}
