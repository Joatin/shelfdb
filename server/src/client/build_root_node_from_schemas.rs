use crate::client::{
    mutation::Mutation,
    query::Query,
    schema::Schema,
};
use futures::{
    stream::BoxStream,
    StreamExt,
};
use juniper::RootNode;
use shelf_database::{
    Cache,
    CacheSchema,
    Schema as DbSchema,
    Store,
};
use std::{
    collections::HashMap,
    sync::Arc,
};

pub async fn build_root_node_from_schemas<'a, C: Cache, S: Store>(
    schemas: BoxStream<'_, C::CacheSchema>,
) -> HashMap<String, Arc<Schema<'a, C, S>>> {
    let list: Vec<_> = schemas
        .then(|i| async move {
            let inner_schema = i.inner_schema().await;
            let name = inner_schema.name.to_string();
            let node = Arc::new(RootNode::new_with_info(
                Query::new(),
                Mutation::new(),
                DbSchema::clone(&inner_schema),
                (),
            ));
            (name, node)
        })
        .collect()
        .await;

    list.into_iter().collect::<HashMap<_, _>>()
}
