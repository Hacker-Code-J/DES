// Import necessary modules and functions.
mod pbox;
mod sbox;
mod keyschedule;
mod utils;

use keyschedule::generate_subkeys;
use utils::{initial_permutation, final_permutation, split_blocks, combine_blocks};

// Constants for Initial and Final Permutations (IP and FP)
const IP: [usize; 64] = [
    // (omitted for brevity, insert actual permutation values)
];

const FP: [usize; 64] = [
    // (omitted for brevity, insert actual permutation values)
];

/// Performs the DES encryption or decryption on a 64-bit block.
///
/// # Arguments
///
/// * `block` - The 64-bit input block.
/// * `key` - The 64-bit DES key.
/// * `encrypt` - A boolean where true means encrypt and false means decrypt.
///
/// # Returns
///
/// * The 64-bit output block after encryption or decryption.
pub fn des(block: u64, key: u64, encrypt: bool) -> u64 {
    // Generate the 16 subkeys
    let subkeys = generate_subkeys(key);
    let subkeys = if encrypt { subkeys } else { subkeys.reverse() };

    // Initial Permutation
    let mut permuted_block = initial_permutation(block, &IP);

    // Split the block into left and right halves
    let (mut left, mut right) = split_blocks(permuted_block);

    // 16 rounds of the Feistel network
    for i in 0..16 {
        let subkey = subkeys[i];
        let new_right = left ^ feistel(right, subkey);
        left = right;
        right = new_right;
    }

    // Combine the blocks and apply the final permutation
    let combined_block = combine_blocks(right, left);  // Note the reversal of right and left
    final_permutation(combined_block, &FP)
}

/// The Feistel (F) function of the DES algorithm.
///
/// # Arguments
///
/// * `right` - The 32-bit right half of the data block.
/// * `subkey` - The 48-bit subkey for this round of the encryption process.
///
/// # Returns
///
/// * The 32-bit result of the Feistel function.
fn feistel(right: u32, subkey: u64) -> u32 {
    // Expansion, substitution, and permutation steps would go here,
    // utilizing the E-box, S-boxes, and P-box. For now, this is left as a placeholder.
    // In actual code, you should implement the expansion, apply the S-boxes,
    // and then apply the P-box permutation.

    let expanded_right = expansion(right); // You need to implement this
    let substituted = sbox::apply_sboxes(expanded_right ^ subkey); // Adjust for actual implementation
    pbox::des_pbox(substituted) // Adjust based on your pbox module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_des_encryption() {
        // Placeholder for DES encryption test
        // Provide actual values for testing
        let block = 0x0123456789ABCDEF;
        let key = 0x133457799BBCDFF1;
        let encrypted = des(block, key, true);
        assert_eq!(encrypted, 0x85E813540F0AB405); // Expected encrypted value
    }

    #[test]
    fn test_des_decryption() {
        // Placeholder for DES decryption test
        // Provide actual values for testing
        let block = 0x85E813540F0AB405;
        let key = 0x133457799BBCDFF1;
        let decrypted = des(block, key, false);
        assert_eq!(decrypted, 0x0123456789ABCDEF); // Expected decrypted value
    }
}
