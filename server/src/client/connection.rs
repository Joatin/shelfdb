use crate::client::edge::Edge;
use crate::client::page_info::PageInfo;
use crate::context::Context;
use juniper::meta::MetaType;
use juniper::{Arguments, DefaultScalarValue, ExecutionResult, Executor, GraphQLType, Registry};
use shelf_database::{Cache, CacheCollection, Document, Schema as DbSchema, Store};
use std::sync::{Arc, RwLockReadGuard};

pub struct Connection<'a, C: Cache, S: Store> {
    edges: Vec<Edge<'a, C, S>>,
    page_info: PageInfo,
    total_count: i32,
}

impl<'a, C: Cache, S: Store> Connection<'a, C, S> {
    pub fn new<Ca: CacheCollection>(cache: &'a RwLockReadGuard<Ca>) -> Self {
        let docs = cache.documents();

        Self {
            edges: docs.iter().take(100).map(|lock| Edge::new(&lock)).collect(),
            page_info: PageInfo {
                has_next_page: false,
                has_previous_page: false,
                start_cursor: "".to_string(),
                end_cursor: "".to_string(),
            },
            total_count: docs.len() as i32,
        }
    }
}

impl<'a, C: Cache, S: Store> GraphQLType for Connection<'a, C, S> {
    type Context = Context<C, S>;
    type TypeInfo = (&'a str, &'a str, &'a DbSchema);

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
                &(&info.0.replace("Connection", "Edge"), info.1, info.2),
            ),
            registry.field::<&PageInfo>("pageInfo", &()),
            registry.field::<&i32>("totalCount", &()),
        ];

        registry
            .build_object_type::<Connection<C, S>>(&info, &fields)
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
            "edges" => executor.resolve_with_ctx(
                &(
                    info.0.replace("Connection", "Edge").as_str(),
                    info.1,
                    info.2,
                ),
                &self.edges,
            ),
            "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
            "totalCount" => executor.resolve_with_ctx(&(), &self.total_count),
            _ => panic!("Field {} not found", field_name),
        }
    }
}

impl<'a, C: Cache, S: Store> From<Vec<Arc<Document>>> for Connection<'a, C, S> {
    fn from(_value: Vec<Arc<Document>>) -> Self {
        unimplemented!()
    }
}
