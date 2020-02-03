use failure::Error;
use futures::Future;
use std::pin::Pin;
use crate::collection::{Schema, Collection};


pub trait Store {

    fn get_schemas(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>>;
    fn save_schema(&self, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    fn get_collections(&self, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<Vec<Collection>, Error>> + Send>>;
    fn save_collection(&self, schema: &Schema, collection: &Collection) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;
}