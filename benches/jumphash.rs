#![feature(test)]
extern crate test;
extern crate jump_consistent_hash as jump_hash;
extern crate seahash;

use jump_hash::{NewHasher, JumpConsistentHash};
use seahash::SeaHasher;

#[derive(Clone, Copy)]
struct NewSeaHasher(u64, u64, u64, u64);
impl NewHasher for NewSeaHasher {
    type Hasher = SeaHasher;
    fn new(&self) -> Self::Hasher {
        SeaHasher::with_seeds(self.0, self.1, self.2, self.3)
    }
}

#[derive(PartialEq, Hash)]
struct Person {
    name: &'static str,
    age: u8,
}

use test::Bencher;

#[bench]
fn bench_2201_100001(b: &mut Bencher) {
    b.iter(|| test::black_box(jump_hash::slot(2201, 100001)));
}

#[bench]
fn bench_10863919174838991_11(b: &mut Bencher) {
    b.iter(|| test::black_box(jump_hash::slot(10863919174838991, 11)));
}

#[bench]
fn bench_alice_11(b: &mut Bencher) {
    let new_hasher = NewSeaHasher(0xe7b0c93ca8525013,
                                  0x011d02b854ae8182,
                                  0x7bcc5cf9c39cec76,
                                  0xfa336285d102d083);
    let hash = JumpConsistentHash::new(11, new_hasher);
    let alice = &Person {
        name: "alice",
        age: 20,
    };
    b.iter(|| test::black_box(hash.get(alice)));
}

macro_rules! bench_slot {
    ( $b:expr, $n:expr ) => {
        {
            let slot = jump_hash::Slot::new($n, seahash::hash);
            $b.iter(|| test::black_box(slot.get(b"alice")));
        }
    }
}

#[bench]
fn bench_slot_8(b: &mut Bencher) {
    bench_slot!(b, 8)
}
#[bench]
fn bench_slot_32(b: &mut Bencher) {
    bench_slot!(b, 32)
}
#[bench]
fn bench_slot_128(b: &mut Bencher) {
    bench_slot!(b, 128)
}
#[bench]
fn bench_slot_512(b: &mut Bencher) {
    bench_slot!(b, 512)
}
#[bench]
fn bench_slot_1024(b: &mut Bencher) {
    bench_slot!(b, 1024)
}
#[bench]
fn bench_slot_2048(b: &mut Bencher) {
    bench_slot!(b, 2048)
}
#[bench]
fn bench_slot_65536(b: &mut Bencher) {
    bench_slot!(b, 65536)
}
