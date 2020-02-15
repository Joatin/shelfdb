use shelf_database::{Schema as DbSchema, Cache, Store};
use std::sync::{Arc, RwLock};
use crate::client::mutation::Mutation;
use juniper::{RootNode};
use crate::client::schema::Schema;
use crate::client::query::Query;
use std::collections::HashMap;
use shelf_database::CacheSchema;

pub fn build_root_node_from_schemas<'a, C: Cache, S: Store>(schemas: &Vec<RwLock<C::CacheSchema>>) -> HashMap<String, Arc<Schema<'a, C, S>>> {
    let mut map = HashMap::with_capacity(schemas.len() + 1);

    for schema_lock in schemas {
        let schema = schema_lock.read().unwrap();
        let name = schema.inner_schema().name.to_string();
        let node = Arc::new(RootNode::new_with_info(Query::new(), Mutation::new(), DbSchema::clone(&schema.inner_schema()), ()));
        map.insert(name, node);
    }

    map
}