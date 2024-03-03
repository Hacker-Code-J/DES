// des.rs
use crate::key_schedule::generate_round_keys;
use crate::utils::{initial_permutation, final_permutation, split_blocks};
use crate::constants::*;

pub fn encrypt(plain_text: &[u8], key: &[u8]) -> Vec<u8> {
    let round_keys = generate_round_keys(key);
    let mut block = initial_permutation(plain_text);
    // Apply the 16 rounds
    for round_key in round_keys.iter() {
        // Encryption logic here
    }
    final_permutation(&block)
}

pub fn decrypt(cipher_text: &[u8], key: &[u8]) -> Vec<u8> {
    let round_keys = generate_round_keys(key).into_iter().rev().collect::<Vec<_>>(); // Reverse for decryption
    let mut block = initial_permutation(cipher_text);
    // Apply the 16 rounds in reverse
    for round_key in round_keys.iter() {
        // Decryption logic here
    }
    final_permutation(&block)
}
