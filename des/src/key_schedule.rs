// key_schedule.rs
use crate::utils::{permute, split_in_half};
use crate::constants::*;

pub fn generate_round_keys(key: &[u8]) -> Vec<Vec<u8>> {
    let key_plus = permute(key, &PC1);
    let (mut left, mut right) = split_in_half(&key_plus);
    let mut round_keys = Vec::new();
    for shift in KEY_SHIFTS.iter() {
        // Key schedule logic here: shift and combine left and right halves, then permute
    }
    round_keys
}
