use failure::Error;
use futures::Future;
use std::pin::Pin;
use crate::{Schema, Collection};


pub trait Store: Sync + Send + 'static {
    fn get_schemas(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>>;
    fn save_schema(&self, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;
}

