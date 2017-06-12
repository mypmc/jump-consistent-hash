#![deny(warnings)]

//! Implements 'Jump Consistent Hash' from the paper
//! [A Fast, Minimal Memory, Consistent Hash Algorithm](http://arxiv.org/abs/1406.2294)
//! by John Lamping, Eric Veach (2014).

const JUMP: u64 = 1 << 31;

use std::hash;

/// Takes a 64 bit key and the number of buckets, outputs a bucket number `0..buckets`.
///
/// # Examples
///
/// ```
/// extern crate jump_consistent_hash as jump;
/// assert_eq!(jump::slot(0, 60), 0);
/// assert_eq!(jump::slot(1, 60), 55);
/// assert_eq!(jump::slot(2, 60), 46);
/// ```
pub fn slot(key: u64, n: usize) -> u32 {
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

pub struct Slot<T: AsRef<[u8]>> {
    pub size: usize,
    hash: Box<Fn(T) -> u64>,
}

impl<T: AsRef<[u8]>> Slot<T> {
    pub fn new<F>(size: usize, hash: F) -> Self
        where F: Fn(T) -> u64 + 'static
    {
        let hash = Box::new(hash);
        Slot { size, hash }
    }

    /// Takes a key, outputs a bucket number `0..buckets`.
    pub fn get(&self, key: T) -> u32 {
        let key = (self.hash)(key);
        self::slot(key, self.size)
    }
}

pub trait NewHasher {
    type Hasher: hash::Hasher;
    fn new(&self) -> Self::Hasher;
}

pub struct JumpConsistentHash<N: NewHasher> {
    pub slots: usize,
    new_hasher: N,
}

impl<N> JumpConsistentHash<N>
    where N: NewHasher
{
    pub fn new(slots: usize, new_hasher: N) -> Self {
        JumpConsistentHash { slots, new_hasher }
    }

    /// Takes a key, outputs a bucket number `0..buckets`.
    pub fn get<T: hash::Hash>(&self, key: &T) -> u32 {
        use hash::Hasher;
        let mut hasher = self.new_hasher.new();
        key.hash(&mut hasher);
        self::slot(hasher.finish(), self.slots)
    }
}
