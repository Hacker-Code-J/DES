/// Performs a circular left shift on a 28-bit number.
///
/// # Arguments
///
/// * `value` - The 28-bit number to be shifted.
/// * `shift` - The number of bits to shift.
///
/// # Returns
///
/// * The shifted 28-bit number.
pub fn circular_left_shift(value: u32, shift: u32) -> u32 {
    // Ensure the value is only 28 bits
    let masked_value = value & 0x0FFFFFFF;  // Masks the value to ensure it is 28 bits
    // Perform the shift
    (masked_value << shift | masked_value >> (28 - shift)) & 0x0FFFFFFF
}

/// Converts an array of bytes into a 64-bit integer.
///
/// # Arguments
///
/// * `bytes` - The array of bytes, must be exactly 8 bytes long.
///
/// # Returns
///
/// * The 64-bit integer.
pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    assert_eq!(bytes.len(), 8, "Input must be exactly 8 bytes long");
    let mut num = 0u64;
    for &byte in bytes {
        num = num << 8 | byte as u64;
    }
    num
}

/// Converts a 64-bit integer into an array of 8 bytes.
///
/// # Arguments
///
/// * `num` - The 64-bit integer.
///
/// # Returns
///
/// * The array of 8 bytes.
pub fn u64_to_bytes(num: u64) -> [u8; 8] {
    [
        (num >> 56) as u8,
        (num >> 48) as u8,
        (num >> 40) as u8,
        (num >> 32) as u8,
        (num >> 24) as u8,
        (num >> 16) as u8,
        (num >> 8) as u8,
        num as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_left_shift() {
        assert_eq!(circular_left_shift(0x1234567, 5), 0x68ACAF2);
        // Add more tests as necessary to cover edge cases and typical scenarios
    }

    #[test]
    fn test_bytes_to_u64_and_back() {
        let original = [1, 2, 3, 4, 5, 6, 7, 8];
        let num = bytes_to_u64(&original);
        let bytes = u64_to_bytes(num);
        assert_eq!(original, bytes, "Conversion to and from u64 should match original bytes");
        // Additional tests can be added to cover more scenarios and edge cases
    }
}