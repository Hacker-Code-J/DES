// Import utilities for bit manipulation.
use crate::utils::{circular_left_shift, u64_to_bytes, bytes_to_u64};

// Constants for key scheduling
const PC1: [usize; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18,
    10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22,
    14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 28, 20, 12, 4
];

const PC2: [usize; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10,
    23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2,
    41, 52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48,
    44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32
];

const SHIFTS: [u32; 16] = [
    1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1
];

/// Generates and returns the 16 subkeys for the DES encryption algorithm.
///
/// # Arguments
///
/// * `key` - The 64-bit DES key.
///
/// # Returns
///
/// * An array of 16 48-bit subkeys.
pub fn generate_subkeys(key: u64) -> [u64; 16] {
    let mut subkeys = [0u64; 16];
    
    // Apply PC1 to the key (dropping parity bits)
    let mut permuted_key = 0u64;
    for i in 0..56 {
        permuted_key |= ((key >> (64 - PC1[i])) & 1) << (55 - i);
    }

    // Split the key into two 28-bit halves
    let mut left = (permuted_key >> 28) & 0x0FFFFFFF;
    let mut right = permuted_key & 0x0FFFFFFF;

    // Generate the 16 subkeys
    for round in 0..16 {
        // Perform circular left shifts
        left = circular_left_shift(left, SHIFTS[round]);
        right = circular_left_shift(right, SHIFTS[round]);

        // Combine halves and apply PC2 to generate subkey
        let combined = (left << 28) | right;
        let mut subkey = 0u64;
        for i in 0..48 {
            subkey |= ((combined >> (56 - PC2[i])) & 1) << (47 - i);
        }
        subkeys[round] = subkey;
    }

    subkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_subkeys() {
        // Example test: Check if the first subkey for a known key is correct.
        // Note: Replace '0x133457799BBCDFF1' with your test key and adjust expected values accordingly.
        let key = 0x133457799BBCDFF1;
        let subkeys = generate_subkeys(key);
        assert_eq!(subkeys[0], 0b000110110000001011101111111111000111000001110010, "First subkey is incorrect");
        // Add more tests as necessary for additional subkeys and different keys
    }
}