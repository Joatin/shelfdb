#![feature(test)]

extern crate test;

use futures::{
    executor::block_on,
    StreamExt,
};
use shelf_database::{
    CacheCollection,
    Collection,
    Document,
};
use shelf_memory_cache::memory_cache_collection::MemoryCacheCollection;
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    iter::FromIterator,
    sync::RwLock as StdRwLock,
};
use test::Bencher;
use tokio::sync::RwLock;
use uuid::Uuid;

#[bench]
fn btree_clone_performance(bencher: &mut Bencher) {
    let data = vec![
        Document {
            id: Uuid::nil(),
            fields: HashMap::new()
        };
        100_000
    ]
    .into_iter()
    .map(|mut i| {
        i.id = Uuid::new_v4();
        (i.id, i)
    })
    .collect::<Vec<_>>();

    let tree = BTreeMap::from_iter(data);

    bencher.iter(|| tree.clone())
}

#[bench]
fn btree_iter_performance(bencher: &mut Bencher) {
    let data = vec![
        Document {
            id: Uuid::nil(),
            fields: HashMap::new()
        };
        100_000
    ]
    .into_iter()
    .map(|mut i| {
        i.id = Uuid::new_v4();
        (i.id, i)
    })
    .collect::<Vec<_>>();

    let tree = BTreeMap::from_iter(data);

    bencher.iter(|| tree.iter().fold(Uuid::nil(), |_, (id, _)| *id))
}

#[bench]
fn tokio_rwlock_read_lock(bencher: &mut Bencher) {
    let lock = RwLock::new(0);

    bencher.iter(|| block_on(lock.read()))
}

#[bench]
fn std_rwlock_read_lock(bencher: &mut Bencher) {
    let lock = StdRwLock::new(0);

    bencher.iter(|| lock.read().unwrap())
}

#[bench]
fn memory_cache_collection_documents_performance_same_data(bencher: &mut Bencher) {
    let data = vec![
        Document {
            id: Uuid::nil(),
            fields: HashMap::new()
        };
        100_000
    ];

    let collection = MemoryCacheCollection::new(Collection::new("TEST".to_string(), None), data);

    bencher.iter(|| {
        block_on(
            block_on(collection.documents())
                .stream()
                .take(100)
                .collect::<Vec<_>>(),
        )
    })
}

#[bench]
fn memory_cache_collection_documents_performance_random_data(bencher: &mut Bencher) {
    let data = vec![
        Document {
            id: Uuid::nil(),
            fields: HashMap::new()
        };
        100_000
    ]
    .into_iter()
    .map(|mut i| {
        i.id = Uuid::new_v4();
        i
    })
    .collect();

    let collection = MemoryCacheCollection::new(Collection::new("TEST".to_string(), None), data);

    bencher.iter(|| {
        block_on(
            block_on(collection.documents())
                .stream()
                .take(100)
                .collect::<Vec<_>>(),
        )
    })
}
