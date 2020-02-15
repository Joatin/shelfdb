use juniper::{GraphQLType, DefaultScalarValue, Registry, ExecutionResult, Arguments, Executor};
use juniper::meta::MetaType;
use crate::client::collection::Collection;
use shelf_database::{Schema as DbSchema, Store, Cache, Document};
use crate::context::Context;
use std::sync::RwLock;

pub struct Edge<'a, C: Cache, S: Store> {
    node: Collection<'a, C, S>,
    cursor: String
}

impl<'a, C: Cache, S: Store> Edge<'a, C, S> {
    pub fn new(lock: &'a RwLock<Document>) -> Self {
        let doc = lock.read().unwrap();
        let cursor = doc.id.to_string();
        Self {
            node: Collection::new(doc),
            cursor
        }
    }
}

impl<'a, C: Cache, S: Store> GraphQLType for Edge<'a, C, S> {
    type Context = Context<C, S>;
    type TypeInfo = (&'a str, &'a str, &'a DbSchema);

    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.0)
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r>
        where DefaultScalarValue: 'r
    {

        let fields  = vec![
            registry.field::<&Collection<C, S>>("node", &(&info.1, &info.2)),
            registry.field::<&String>("cursor", &())
        ];

        registry.build_object_type::<Edge<C, S>>(&info, &fields).into_meta()
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _args: &Arguments,
        executor: &Executor<Self::Context>
    )
        -> ExecutionResult
    {
        match field_name {
            "node" => executor.resolve_with_ctx(&(info.1, info.2), &self.node),
            "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
            _ => panic!("Field {} not found", field_name)
        }
    }
}