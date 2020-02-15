use crate::{Document, Collection};
use uuid::Uuid;
use std::sync::RwLock;
use std::ops::Deref;

pub trait CacheCollection: 'static + Send + Sync {
    fn set_document(&mut self, document: Document);
    fn inner_collection(&self) -> &Collection;
    fn inner_collection_mut(&mut self) -> &mut Collection;
    fn documents(&self) -> &[RwLock<Document>];
    fn document(&self, id: Uuid) -> Option<&RwLock<Document>>;
    fn find_first_by_field(&self, field_name: &str, field_value: &str) -> Option<&RwLock<Document>>;
    fn find_by_field(&self, field_name: &str, field_value: &str) -> Vec<&RwLock<Document>>;
}


impl Deref for dyn CacheCollection {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.inner_collection()
    }
}