use crate::memory_document_result::MemoryDocumentResult;
use futures::{
    future::BoxFuture,
    FutureExt,
    StreamExt,
};
use shelf_database::{
    CacheCollection,
    Collection,
    Document,
    DocumentResult,
};
use std::{
    collections::BTreeMap,
    mem,
    sync::Arc,
};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryCacheCollection {
    collection: Arc<RwLock<Collection>>,
    documents: Arc<RwLock<Vec<Arc<Document>>>>,
    id_index: Arc<RwLock<BTreeMap<Uuid, Arc<Document>>>>,
}

impl MemoryCacheCollection {
    pub fn new(collection: Collection, documents: Vec<Document>) -> Self {
        let docs: Vec<_> = documents.into_iter().map(Arc::new).collect();

        let id_index: BTreeMap<_, _> = docs.clone().into_iter().map(|i| (i.id, i)).collect();

        Self {
            collection: Arc::new(RwLock::new(collection)),
            documents: Arc::new(RwLock::new(docs)),
            id_index: Arc::new(RwLock::new(id_index)),
        }
    }

    pub(crate) async fn get_size(&self) -> usize {
        let mut size =
            self.id_index.read().await.len() * (mem::size_of::<Uuid>() + mem::size_of::<usize>());

        size = self
            .documents()
            .await
            .stream()
            .fold(size, |acc, val| async move { (acc + val.get_size()) + 0 })
            .await;

        size += self.collection.read().await.get_size();

        size
    }
}

impl CacheCollection for MemoryCacheCollection {
    // this is crazy slow...
    fn set_document(&self, document: Document) -> BoxFuture<()> {
        async move {
            let mut index = self.id_index.write().await;

            match index.get(&document.id) {
                Some(_val) => {
                    // let mut lock = self.documents.write().await;
                    // lock[*index] = Arc::new(document);
                }
                None => {
                    let doc = Arc::new(document);
                    index.insert(doc.id, doc.clone());

                    let mut lock = self.documents.write().await;
                    lock.push(doc);
                }
            }
        }
        .boxed()

        //        match self.id_index.binary_search_by_key(&document.id, |i|
        // i.0) {            Ok(key) => {
        //                let index = self.id_index[key].1;
        //                self.documents[index] = RwLock::new(document);
        //            },
        //            Err(key) => {
        //                // The document does not exist, we just need to insert
        // it                self.id_index.insert(key, (document.id,
        // self.documents.len()));
        // self.documents.push(RwLock::new(document));
        //
        //            },
        //        }
    }

    fn inner_collection(&self) -> BoxFuture<Collection> {
        async move { self.collection.read().await.clone() }.boxed()
    }

    fn set_collection(&self, collection: Collection) -> BoxFuture<()> {
        async move {
            let mut lock = self.collection.write().await;
            *lock = collection;
        }
        .boxed()
    }

    fn documents<'a>(&'a self) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>> {
        async move {
            let lock = self.id_index.read().await;
            Box::new(MemoryDocumentResult::new(lock)) as Box<dyn DocumentResult>
        }
        .boxed()
    }

    fn document(&self, id: Uuid) -> BoxFuture<Option<Arc<Document>>> {
        async move {
            let index = self.id_index.read().await;
            match index.get(&id) {
                None => None,
                Some(val) => Some(Arc::clone(&val)),
            }
        }
        .boxed()
    }

    fn find_first_by_field<'a>(
        &'a self,
        field_name: &'a str,
        field_value: &'a str,
    ) -> BoxFuture<'a, Option<Arc<Document>>> {
        async move {
            let docs = self.find_by_field(field_name, field_value).await;
            let stream = docs.stream();
            stream.into_future().map(|(next, _)| next).await
        }
        .boxed()
    }

    // TODO: Might be good to do some index checking ;)
    fn find_by_field<'a>(
        &'a self,
        _field_name: &'a str,
        _field_value: &'a str,
    ) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>> {
        async move {
            let lock = self.id_index.read().await;
            Box::new(MemoryDocumentResult::new(lock)) as Box<dyn DocumentResult>
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use crate::memory_cache_collection::MemoryCacheCollection;
    use shelf_database::{
        CacheCollection,
        Collection,
    };

    #[tokio::test]
    async fn inner_collection_should_return_the_inner_collection() {
        let cache = MemoryCacheCollection::new(Collection::new("TEST".to_string(), None), vec![]);
        assert_eq!(cache.inner_collection().await.name, "TEST");
    }
}
