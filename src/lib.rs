#![deny(warnings)]

//! Implements 'Jump Consistent Hash' from the paper
//! [A Fast, Minimal Memory, Consistent Hash Algorithm](http://arxiv.org/abs/1406.2294)
//! by John Lamping, Eric Veach (2014).

const JUMP: u64 = 1 << 31;

/// Takes a 64 bit key and the number of buckets, outputs a bucket number `0..buckets`.
///
/// # Examples
///
/// ```
/// extern crate jump_consistent_hash as jch;
/// assert_eq!(jch::hash(0, 60), 0);
/// assert_eq!(jch::hash(1, 60), 55);
/// assert_eq!(jch::hash(2, 60), 46);
/// ```
///
pub fn hash(key: u64, n: usize) -> u32 {
    let len = if n == 0 { 1 } else { n as i64 };
    let mut k = key;
    let mut b = -1;
    let mut j = 0;
    while j < len {
        b = j;
        k = k.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b + 1) as f64 * (JUMP as f64 / ((k >> 33) + 1) as f64)) as i64;
    }
    b as u32
}
