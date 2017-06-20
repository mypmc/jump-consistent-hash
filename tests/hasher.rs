extern crate jump_consistent_hash;
extern crate seahash;

use jump_consistent_hash::{NewHasher, JumpConsistentHash};
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

#[test]
fn hasher_test() {
    let new_hasher = NewSeaHasher(
        0xe7b0c93ca8525013,
        0x011d02b854ae8182,
        0x7bcc5cf9c39cec76,
        0xfa336285d102d083,
    );
    let hash1 = JumpConsistentHash::new(11, new_hasher);
    let hash2 = JumpConsistentHash::new(10, new_hasher);
    let hash3 = JumpConsistentHash::new(11, new_hasher);

    let alice = &Person {
        name: "alice",
        age: 20,
    };
    let bob = &Person {
        name: "bob",
        age: 30,
    };

    assert_eq!(7, hash1.get(alice));
    assert_eq!(6, hash1.get(bob));

    assert_eq!(7, hash2.get(alice));
    assert_eq!(6, hash2.get(bob));

    assert_eq!(7, hash3.get(alice));
    assert_eq!(6, hash3.get(bob));
}
