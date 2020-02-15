use shelf_database::{Collection, CacheCollection, Document};
use uuid::Uuid;
use std::sync::RwLock;
use std::collections::BTreeMap;


pub struct MemoryCacheCollection {
    collection: Collection,
    documents: Vec<RwLock<Document>>,
    id_index: BTreeMap<Uuid, usize>
}

impl MemoryCacheCollection {

    pub fn new(collection: Collection, documents: Vec<Document>) -> Self {
        Self {
            collection,
            documents: documents.into_iter().map(RwLock::new).collect(),
            id_index: BTreeMap::new()
        }
    }

    pub fn get_data_cloned(&self) -> (Collection, Vec<Document>) {
        let documents: Vec<Document> = self.documents().iter().map(|lock| {
            let doc = lock.read().unwrap();
            doc.clone()
        }).collect();
        (self.collection.clone(), documents)
    }
}

impl CacheCollection for MemoryCacheCollection {

    // this is crazy slow...
    fn set_document(&mut self, document: Document) {
        match self.id_index.get(&document.id) {
            Some(index) => {
                self.documents[*index] = RwLock::new(document);
            },
            None => {
                self.id_index.insert(document.id, self.documents.len());
                self.documents.push(RwLock::new(document));
            },
        }
//        match self.id_index.binary_search_by_key(&document.id, |i| i.0) {
//            Ok(key) => {
//                let index = self.id_index[key].1;
//                self.documents[index] = RwLock::new(document);
//            },
//            Err(key) => {
//                // The document does not exist, we just need to insert it
//                self.id_index.insert(key, (document.id, self.documents.len()));
//                self.documents.push(RwLock::new(document));
//
//            },
//        }
    }

    fn inner_collection(&self) -> &Collection {
        &self.collection
    }

    fn inner_collection_mut(&mut self) -> &mut Collection {
        &mut self.collection
    }

    fn documents(&self) -> &[RwLock<Document>] {
        &self.documents
    }

    fn document(&self, id: Uuid) -> Option<&RwLock<Document>> {
        self.id_index.get(&id).map(|i| &self.documents[*i])
    }

    fn find_first_by_field(&self, field_name: &str, field_value: &str) -> Option<&RwLock<Document>> {
        self.documents.iter().find(|i| {
            let lock = i.read().unwrap();
            if let Some(val) = lock.fields.get(field_name) {
                if val == field_value {
                    return true
                }
            }
            false
        })
    }

    fn find_by_field(&self, field_name: &str, field_value: &str) -> Vec<&RwLock<Document>> {
        self.documents.iter().filter(|i| {
            let lock = i.read().unwrap();
            if let Some(val) = lock.fields.get(field_name) {
                if val == field_value {
                    return true
                }
            }
            false
        }).collect()
    }
}

impl Into<Box<dyn CacheCollection>> for Box<MemoryCacheCollection> {
    fn into(self) -> Box<dyn CacheCollection> {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::memory_cache_collection::MemoryCacheCollection;
    use shelf_database::{Collection, CacheCollection};

    #[test]
    fn inner_collection_should_return_the_inner_collection() {
        let cache = MemoryCacheCollection::new(Collection::new("TEST".to_string(), None), vec![]);
        assert_eq!(cache.inner_collection().name, "TEST");
    }

    #[test]
    fn inner_collection_mut_should_return_the_inner_collection() {
        let mut cache = MemoryCacheCollection::new(Collection::new("TEST".to_string(), None), vec![]);
        assert_eq!(cache.inner_collection_mut().name, "TEST");
    }
}