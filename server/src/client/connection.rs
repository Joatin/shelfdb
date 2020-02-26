use crate::{
    client::{
        edge::Edge,
        page_info::PageInfo,
    },
    context::Context,
};
use futures::{
    future::BoxFuture,
    FutureExt,
    StreamExt,
};
use juniper::{
    meta::MetaType,
    Arguments,
    DefaultScalarValue,
    ExecutionResult,
    Executor,
    GraphQLType,
    GraphQLTypeAsync,
    Registry,
};
use shelf_database::{
    Cache,
    CacheCollection,
    Document,
    Schema as DbSchema,
    Store,
};
use std::sync::Arc;

pub struct Connection<C: Cache, S: Store> {
    edges: Vec<Edge<C, S>>,
    page_info: PageInfo,
    total_count: i32,
}

impl<C: Cache, S: Store> Connection<C, S> {
    pub async fn new<Ca: CacheCollection>(cache: Ca) -> Connection<C, S> {
        let edges: Vec<Edge<C, S>> = {
            let docs = cache.documents().await;
            let stream = docs.stream();
            stream.take(100).then(Edge::new).collect().await
        };

        // let edges: Vec<Edge<C, S>> = ;
        // let total = docs.total();

        Self {
            edges,
            page_info: PageInfo {
                has_next_page: false,
                has_previous_page: false,
                start_cursor: "".to_string(),
                end_cursor: "".to_string(),
            },
            // TODO: We need to get total count without exhausting the stream
            total_count: 0 as i32,
        }
    }
}

impl<C: Cache, S: Store> GraphQLType for Connection<C, S> {
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
            registry.field::<&Vec<Edge<C, S>>>(
                "edges",
                &(
                    info.0.replace("Connection", "Edge"),
                    info.1.clone(),
                    info.2.clone(),
                ),
            ),
            registry.field::<&PageInfo>("pageInfo", &()),
            registry.field::<&i32>("totalCount", &()),
        ];

        registry
            .build_object_type::<Connection<C, S>>(&info, &fields)
            .into_meta()
    }
}

impl<C: Cache, S: Store> GraphQLTypeAsync<DefaultScalarValue> for Connection<C, S> {
    fn resolve_field_async<'r>(
        &'r self,
        info: &'r Self::TypeInfo,
        field_name: &'r str,
        _args: &'r Arguments,
        executor: &'r Executor<Self::Context>,
    ) -> BoxFuture<ExecutionResult> {
        async move {
            match field_name {
                "edges" => executor.resolve_with_ctx(
                    &(
                        info.0.replace("Connection", "Edge"),
                        info.1.clone(),
                        info.2.clone(),
                    ),
                    &self.edges,
                ),
                "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
                "totalCount" => executor.resolve_with_ctx(&(), &self.total_count),
                _ => panic!("Field {} not found", field_name),
            }
        }
        .boxed()
    }
}

impl<C: Cache, S: Store> From<Vec<Arc<Document>>> for Connection<C, S> {
    fn from(_value: Vec<Arc<Document>>) -> Self {
        unimplemented!()
    }
}
