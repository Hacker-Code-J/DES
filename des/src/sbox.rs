// S-boxes used in DES (Data Encryption Standard).
// Each S-box is a 4x16 matrix represented as a 2D array.
// Here, we're defining just one S-box for illustration.
// In actual implementation, you should define all eight S-boxes.
const S_BOX_1: [[u8; 16]; 4] = [
    [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
    [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
    [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
    [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
];

/// Applies the S-box substitution on a 6-bit input to produce a 4-bit output.
///
/// # Arguments
///
/// * `input` - The 6-bit input block to be substituted.
///
/// # Returns
///
/// * The 4-bit output block after substitution.
pub fn apply_sbox(input: u8) -> u8 {
    // Validate that the input is indeed 6 bits
    assert!(input < 64, "S-box input must be a 6-bit value");

    // Determine which part of the S-box to use
    let row = ((input & 0b100000) >> 4) | (input & 0b1);
    let col = (input & 0b011110) >> 1;

    // Perform the substitution
    S_BOX_1[row as usize][col as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_sbox() {
        // This is a simple test to ensure that the S-box substitution is working correctly.
        // The actual values for input and expected need to be adjusted based on the specific S-box being tested.
        let input = 0b101010; // Example input, replace with actual
        let expected = 0b0010; // Expected output from S_BOX_1, replace with actual
        let result = apply_sbox(input);
        assert_eq!(result, expected, "S-box substitution failed.");
    }
}