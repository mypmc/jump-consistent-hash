extern crate jump_consistent_hash as jump;

use std::collections::BTreeMap;

#[test]
#[ignore]
fn simulate_rebalance() {
    const N: usize = 24;
    for i in 0..10 {
        simulate("jmp", N, N + i + 1, jump::hash);
        simulate("mod", N, N + i + 1, |key, len| (key % (len as u64)) as u32);
    }
}

fn simulate<F>(name: &'static str, before: usize, after: usize, func: F)
    where F: Fn(u64, usize) -> u32
{
    let mut map1 = BTreeMap::new();
    let mut map2 = BTreeMap::new();
    let mut moves = 0;

    for i in 0..1_000_000 {
        let b1 = func(i, before);
        let mut n = map1.entry(i).or_insert(0);
        *n += 1;

        let b2 = func(i, after);
        let mut m = map2.entry(i).or_insert(0);
        *m += 1;

        if b1 != b2 {
            moves += 1;
        }
    }

    assert_eq!(map1.values().sum::<u32>(), map2.values().sum::<u32>());

    println!("{} {:>2}->{:<3} {:>6}", name, before, after, moves);
}
