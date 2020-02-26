use crate::{
    client::collection::Collection,
    context::Context,
};
use juniper::{
    meta::MetaType,
    Arguments,
    DefaultScalarValue,
    ExecutionResult,
    Executor,
    GraphQLType,
    Registry,
};
use shelf_database::{
    Cache,
    Document,
    Schema as DbSchema,
    Store,
};
use std::sync::Arc;

pub struct Edge<C: Cache, S: Store> {
    node: Collection<C, S>,
    cursor: String,
}

impl<C: Cache, S: Store> Edge<C, S> {
    pub async fn new(doc: Arc<Document>) -> Edge<C, S> {
        let cursor = doc.id.to_string();
        Self {
            node: Collection::new(doc),
            cursor,
        }
    }
}

impl<C: Cache, S: Store> GraphQLType for Edge<C, S> {
    type Context = Context<C, S>;
    type TypeInfo = (String, String, DbSchema);

    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.0)
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r>
    where
        DefaultScalarValue: 'r,
    {
        let fields = vec![
            registry.field::<&Collection<C, S>>("node", &(info.1.clone(), info.2.clone())),
            registry.field::<&String>("cursor", &()),
        ];

        registry
            .build_object_type::<Edge<C, S>>(&info, &fields)
            .into_meta()
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _args: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        match field_name {
            "node" => executor.resolve_with_ctx(&(info.1.clone(), info.2.clone()), &self.node),
            "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
            _ => panic!("Field {} not found", field_name),
        }
    }
}
