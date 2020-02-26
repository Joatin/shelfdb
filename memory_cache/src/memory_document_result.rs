use futures::{
    stream,
    stream::BoxStream,
    StreamExt,
};
use shelf_database::{
    Document,
    DocumentResult,
};
use std::{
    collections::BTreeMap,
    sync::Arc,
};
use tokio::sync::RwLockReadGuard;
use uuid::Uuid;

pub struct MemoryDocumentResult<'a> {
    lock: RwLockReadGuard<'a, BTreeMap<Uuid, Arc<Document>>>,
}

impl<'a> MemoryDocumentResult<'a> {
    pub fn new(lock: RwLockReadGuard<'a, BTreeMap<Uuid, Arc<Document>>>) -> Self {
        Self { lock }
    }
}

impl<'a> DocumentResult for MemoryDocumentResult<'a> {
    fn total(&self) -> usize {
        self.lock.len()
    }

    fn stream(&self) -> BoxStream<Arc<Document>> {
        let iter = self.lock.iter();
        stream::iter(iter)
            .map(|(_key, val)| Arc::clone(&val))
            .boxed()
    }
}
