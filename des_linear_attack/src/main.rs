fn main() {
    let s: Vec<u8> = vec![
        0x2, 0xe, 0xc, 0xb, 0x4, 0x2, 0x1, 0xc, 0x7, 0x4, 0xa, 0x7, 0xb, 0xd, 0x6, 0x1,
        0x8, 0x5, 0x5, 0x0, 0x3, 0xf, 0xf, 0xa, 0xd, 0x3, 0x0, 0x9, 0xe, 0x8, 0x9, 0x6,
        0x4, 0xb, 0x2, 0x8, 0x1, 0xc, 0xb, 0x7, 0xa, 0x1, 0xd, 0xe, 0x7, 0x2, 0x8, 0xd,
        0xf, 0x6, 0x9, 0xf, 0xc, 0x0, 0x5, 0x9, 0x6, 0xa, 0x3, 0x4, 0x0, 0x5, 0xe, 0x3
    ];
    const NUM_TEXTS: usize = 1 << 6; // 64 inputs for 6-bit S-box
    let mut key_guess = [0; 2];
    let mut max_difference = 0;

    // Iterate through all possible 6-bit keys for S5
    for key in 0..64u8 {
        let mut count = 0;

        // Simulate the input through S5 for each possible input
        for input in 0..64u8 { // S5 has 6 bits input
            let output = s[(input ^ key) as usize]; // XOR input with the key as S5's input

            // Compute the linear approximation: X[4] (fifth bit) ⊕ Output bits
            let approximation = ((input >> 4) & 1) ^ ((output >> 1) & 1) ^ ((output >> 3) & 1) ^ ((output >> 4) & 1) ^ ((output >> 5) & 1);

            if approximation == 0 {
                count += 1;
            }
        }

        // Calculate how far this result deviates from random
        let difference = ((count as i32) - (NUM_TEXTS as i32 / 2)).abs();
        
        if difference > max_difference {
            max_difference = difference;
            key_guess = [key, 64 - count]; // NUM_TEXTS is 64 for a 6-bit S-box
        }
    }

    println!("Most likely key for the bit position: {}", key_guess[0]);
    println!("Second most likely key for the bit position: {}", key_guess[1]);
}


// fn main() {
//     let s: Vec<u8> = vec![
//         0x2, 0xe, 0xc, 0xb, 0x4, 0x2, 0x1, 0xc, 0x7, 0x4, 0xa, 0x7, 0xb, 0xd, 0x6, 0x1,
//         0x8, 0x5, 0x5, 0x0, 0x3, 0xf, 0xf, 0xa, 0xd, 0x3, 0x0, 0x9, 0xe, 0x8, 0x9, 0x6,
//         0x4, 0xb, 0x2, 0x8, 0x1, 0xc, 0xb, 0x7, 0xa, 0x1, 0xd, 0xe, 0x7, 0x2, 0x8, 0xd,
//         0xf, 0x6, 0x9, 0xf, 0xc, 0x0, 0x5, 0x9, 0x6, 0xa, 0x3, 0x4, 0x0, 0x5, 0xe, 0x3
//     ];
//     const NUM_TEXTS: usize = 1 << 16; // Adjust as needed
//     let mut approximations = vec![0; NUM_TEXTS];
//     let mut key_guess = [0; 2];
//     let mut max_difference = 0.0;

//     // Iterate through all possible 6-bit keys for S5
//     for key in 0..64 {
//         let mut count = 0;

//         // Simulate the input through S5 for each possible input
//         for input in 0..64 { // S5 has 6 bits input
//             let output = s[input ^ key]; // XOR input with the key as S5's input

//             // Compute the linear approximation: X[15] ⊕ P(X ⊕ K)[7,18,24,29] = K[22]
//             // Since we are not implementing full DES, assume direct correlation for demonstration:
//             let approximation = (input >> 4) & 1 ^ ((output >> 1) & 1) ^ ((output >> 3) & 1) ^ ((output >> 4) & 1) ^ ((output >> 5) & 1);

//             if approximation == 0 {
//                 count += 1;
//             }
//         }

//         // Calculate how far this result deviates from random
//         let difference = ((count as f32) - (NUM_TEXTS as f32 / 2.0)).abs();
//         approximations[key as usize] = count;

//         if difference > max_difference {
//             max_difference = difference;
//             key_guess = [key, NUM_TEXTS - count];
//         }
//     }

//     println!("Most likely key for the bit position: {}", key_guess[0]);
//     println!("Second most likely key for the bit position: {}", key_guess[1]);
// }


// const NUM_TEXTS: usize = 1 << 16; // Number of plaintexts; adjust as needed
// const KEY_BIT: usize = 22;        // The bit of the key we're trying to find
// const S5_TABLE: [[u8; 4]; 64] = [
//     0x2, 0xe, 0xc, 0xb, 0x4, 0x2, 0x1, 0xc, 0x7, 0x4, 0xa, 0x7, 0xb, 0xd, 0x6, 0x1,
//     0x8, 0x5, 0x5, 0x0, 0x3, 0xf, 0xf, 0xa, 0xd, 0x3, 0x0, 0x9, 0xe, 0x8, 0x9, 0x6,
//     0x4, 0xb, 0x2, 0x8, 0x1, 0xc, 0xb, 0x7, 0xa, 0x1, 0xd, 0xe, 0x7, 0x2, 0x8, 0xd,
//     0xf, 0x6, 0x9, 0xf, 0xc, 0x0, 0x5, 0x9, 0x6, 0xa, 0x3, 0x4, 0x0, 0x5, 0xe, 0x3
// ];

// fn main() {
//     let mut approximations = vec![0; NUM_TEXTS];
//     let mut key_guess = [0; 2];
//     let mut max_difference = 0.0;

//     // Iterate through all possible keys for the S5 box
//     for key in 0..64 { // S5 has 6 bits keys
//         let mut count = 0;

//         // Go through each plaintext
//         for plaintext in 0..NUM_TEXTS {
//             // Simulate encryption under the current subkey
//             // Normally, we'd extract the relevant bits from the full plaintext,
//             // but for simplicity, assume plaintext is directly the input to S5
//             let input = (plaintext >> 1) & 0b111111; // Adjust based on actual bit positions
//             let output = S5_TABLE[input as usize][key as usize];

//             // Compute linear approximation
//             let approximation = (plaintext >> 15) & 1 ^ (output >> 2 & 1 ^ output >> 3 & 1 ^ output >> 4 & 1 ^ output >> 5 & 1);

//             if approximation == 0 {
//                 count += 1;
//             }
//         }

//         // Calculate how far this result deviates from random
//         let difference = ((count as f32) - (NUM_TEXTS as f32 / 2.0)).abs();
//         approximations[key as usize] = count;

//         if difference > max_difference {
//             max_difference = difference;
//             key_guess = [key, NUM_TEXTS - count];
//         }
//     }

//     println!("Most likely key for bit {}: {}", KEY_BIT, key_guess[0]);
//     println!("Second most likely key for bit {}: {}", KEY_BIT, key_guess[1]);
// }

// mod sbox_lat;
// use sbox_lat::{display, lat};

// fn main() {
//     let s: Vec<u8> = vec![
//         0x2, 0xe, 0xc, 0xb, 0x4, 0x2, 0x1, 0xc, 0x7, 0x4, 0xa, 0x7, 0xb, 0xd, 0x6, 0x1,
//         0x8, 0x5, 0x5, 0x0, 0x3, 0xf, 0xf, 0xa, 0xd, 0x3, 0x0, 0x9, 0xe, 0x8, 0x9, 0x6,
//         0x4, 0xb, 0x2, 0x8, 0x1, 0xc, 0xb, 0x7, 0xa, 0x1, 0xd, 0xe, 0x7, 0x2, 0x8, 0xd,
//         0xf, 0x6, 0x9, 0xf, 0xc, 0x0, 0x5, 0x9, 0x6, 0xa, 0x3, 0x4, 0x0, 0x5, 0xe, 0x3
//     ];
//     display::print_lat(&s, 6, 4);
// }