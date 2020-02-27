use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
};

pub fn compute_hash_sum(data: &str) -> u64 {
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    s.finish()
}
