use crate::{
    Collection,
    Document,
    DocumentResult,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use uuid::Uuid;

pub trait CacheCollection: 'static + Send + Sync + Clone {
    fn set_document(&self, document: Document) -> BoxFuture<()>;
    fn inner_collection(&self) -> BoxFuture<Collection>;
    fn set_collection(&self, collection: Collection) -> BoxFuture<()>;
    fn documents<'a>(&'a self) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>>;
    fn document(&self, id: Uuid) -> BoxFuture<Option<Arc<Document>>>;
    fn find_first_by_field<'a>(
        &'a self,
        field_name: &'a str,
        field_value: &'a str,
    ) -> BoxFuture<'a, Option<Arc<Document>>>;
    fn find_by_field<'a>(
        &'a self,
        field_name: &'a str,
        field_value: &'a str,
    ) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>>;
}
