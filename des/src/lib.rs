// lib.rs
mod des;
mod key_schedule;
mod utils;
mod constants;

pub use des::{encrypt, decrypt};