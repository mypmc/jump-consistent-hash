extern crate seahash;
extern crate jump_consistent_hash;
use jump_consistent_hash::hash as jump;

#[test]
#[ignore]
fn simulate_rebalance() {
    let mut n = 1;
    for i in 1..10 {
        simulate(" mod", n, n + 1, bymod);
        simulate("slot", n, n + 1, slot);
        simulate("jump", n, n + 1, jump);
        n += i;
    }
}

fn seeded<T: AsRef<[u8]>>(key: T) -> u64 {
    seahash::hash_seeded(key.as_ref(),
                         0x16f11fe89b0d677c,
                         0xb480a793d8e6c86c,
                         0x6fe2e5aaf078ebc9,
                         0x14f994a4c5259381)
}

fn slot(key: u64, len: usize) -> u32 {
    jump_consistent_hash::Slot::new(len, seeded).get(format!("{}", key))
}

fn bymod(key: u64, len: usize) -> u32 {
    (key % (len as u64)) as u32
}

fn simulate<F>(name: &'static str, before: usize, after: usize, func: F)
    where F: Fn(u64, usize) -> u32
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
