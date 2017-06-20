extern crate seahash;
extern crate jump_consistent_hash;
use jump_consistent_hash::slot;

#[test]
#[ignore]
fn simulate_rebalance() {
    let mut n = 1;
    for i in 1..10 {
        simulate(" mod", n, n + 1, bymod);
        simulate("slot", n, n + 1, slot);
        n += i;
    }
}

fn bymod(key: u64, len: usize) -> u32 {
    (key % (len as u64)) as u32
}

fn simulate<F>(name: &'static str, before: usize, after: usize, func: F)
where
    F: Fn(u64, usize) -> u32,
{
    let mut moves = 0;

    for i in 0..65536 {
        let b1 = func(i, before);
        let b2 = func(i, after);
        if b1 != b2 {
            moves += 1;
        }
    }
    println!("{:>5} {:>2}->{:<3} {:>6}", name, before, after, moves);
}
