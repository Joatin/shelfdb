use crate::Document;
use futures::stream::BoxStream;
use std::sync::Arc;

pub trait DocumentResult: Send + Sync {
    fn total(&self) -> usize;
    fn stream(&self) -> BoxStream<Arc<Document>>;
}
