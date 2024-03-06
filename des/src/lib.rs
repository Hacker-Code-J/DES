// lib.rs - The entry point of the DES encryption library

// Declare the modules used in the DES implementation.
// The `mod` keyword is used to declare a module, and by default, these are private.
// You can make functions or structs public with the `pub` keyword.
mod des;
mod pbox;
mod sbox;
mod keyschedule;
mod utils; // Assuming you have utility functions

// Re-export the primary functions and types that should be public.
// This way, users of your library can access these functions directly through the library root.
pub use des::{encrypt, decrypt};
pub use keyschedule::{generate_subkeys};

// Optionally, you could also expose the individual components for users who may want to use them directly.
// For example, you could allow users to access the P-box and S-box functions directly, but that is often not necessary
// for a higher-level encryption library.
// However, if you do want to expose them, you could do it like this:
// pub use pbox::des_pbox;
// pub use sbox::apply_sbox;

#[cfg(test)]
mod tests {
    // Here you can add integration tests for your library.
    // These tests would typically use multiple components of your library to test its overall functionality.

    use super::*;

    #[test]
    fn test_des_encryption_decryption() {
        let key = DESKey::new([0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1]); // Example key
        let plaintext = 0x0123456789ABCDEF; // Example plaintext
        let subkeys = generate_subkeys(&key);

        let ciphertext = encrypt(plaintext, &subkeys);
        let decrypted_text = decrypt(ciphertext, &subkeys);

        assert_eq!(plaintext, decrypted_text, "DES decryption failed to revert to original plaintext");
    }
}