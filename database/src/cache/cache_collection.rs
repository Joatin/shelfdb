use crate::{
    Collection,
    Document,
};
use futures::{
    future::BoxFuture,
    stream::BoxStream,
};
use uuid::Uuid;

pub trait CacheCollection: 'static + Send + Sync + Clone {
    fn set_document(&self, document: Document) -> BoxFuture<()>;
    fn inner_collection(&self) -> BoxFuture<Collection>;
    fn set_collection(&self, collection: Collection) -> BoxFuture<()>;
    fn documents(&self) -> BoxStream<Document>;
    fn document(&self, id: Uuid) -> BoxFuture<Option<Document>>;
    fn find_first_by_field<'a>(
        &'a self,
        field_name: &'a str,
        field_value: &'a str,
    ) -> BoxFuture<'a, Option<Document>>;
    fn find_by_field<'a>(
        &'a self,
        field_name: &'a str,
        field_value: &'a str,
    ) -> BoxStream<'a, Document>;
}
