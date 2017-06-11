#![feature(test)]
extern crate test;
extern crate jump_consistent_hash as jump_hash;
extern crate seahash;

use test::Bencher;

#[bench]
fn bench_2201_100001(b: &mut Bencher) {
    b.iter(|| test::black_box(jump_hash::hash(2201, 100001)));
}

#[bench]
fn bench_10863919174838991_11(b: &mut Bencher) {
    b.iter(|| test::black_box(jump_hash::hash(10863919174838991, 11)));
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
