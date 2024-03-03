const PC1: [usize; 56] = [ 
    57, 49, 41, 33, 25, 17,  9,
    1, 58, 50, 42, 34, 26, 18,
    10,  2, 59, 51, 43, 35, 27,
    19, 11,  3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15,
    7, 62, 54, 46, 38, 30, 22,
    14,  6, 61, 53, 45, 37, 29,
    21, 13,  5, 28, 20, 12,  4
];
const PC2: [usize; 48] = [
    14, 17, 11, 24,  1,  5,
    3, 28, 15,  6, 21, 10,
    23, 19, 12,  4, 26,  8,
    16,  7, 27, 20, 13,  2,
    41, 52, 31, 37, 47, 55,
    30, 40, 51, 45, 33, 48,
    44, 49, 39, 56, 34, 53,
    46, 42, 50, 36, 29, 32
];
const SHIFTS: [usize; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

fn generate_subkeys(key: u64) -> Vec<u64> {
    let mut subkeys = Vec::new();
    // Initial permutation using PC-1
    let mut permuted_choice_1 = 0;
    for &index in PC1.iter() {
        permuted_choice_1 <<= 1;
        permuted_choice_1 |= (key >> (64 - index)) & 1;
    }
    
    // Splitting into two halves
    let mut left = permuted_choice_1 >> 28;
    let mut right = permuted_choice_1 & 0x0FFFFFFF;

    // Generating 16 subkeys
    for &shifts in SHIFTS.iter() {
        left = ((left << shifts) | (left >> (28 - shifts))) & 0x0FFFFFFF;
        right = ((right << shifts) | (right >> (28 - shifts))) & 0x0FFFFFFF;

        // Combining halves and applying PC-2
        let combined = (left << 28) | right;
        let mut subkey = 0;
        for &index in PC2.iter() {
            subkey <<= 1;
            subkey |= (combined >> (56 - index)) & 1;
        }
        subkeys.push(subkey);
    }

    subkeys
}


fn main() {
    let key = 0x133457799BBCDFF1; // Example key
    let subkeys = generate_subkeys(key);
    for (i, subkey) in subkeys.iter().enumerate() {
        println!("Round {}: {:X}", i + 1, subkey);
    }
}

