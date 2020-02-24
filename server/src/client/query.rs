use crate::client::connection::Connection;
use crate::client::node::Node;
use crate::client::query_field::QueryField;
use crate::context::Context;
use failure::_core::marker::PhantomData;
use futures::FutureExt;
use juniper::meta::MetaType;
use juniper::{
    Arguments, BoxFuture, DefaultScalarValue, ExecutionResult, Executor, FieldError, GraphQLType,
    GraphQLTypeAsync, Registry,
};
use shelf_database::CacheSchema;
use shelf_database::{Cache, Schema as DbSchema, Store};
use std::sync::RwLockReadGuard;

pub struct Query<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Query<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData,
        }
    }

    fn resolve_collection(
        &self,
        _context: &Context<C, S>,
        _args: &Arguments,
        _executor: &Executor<Context<C, S>>,
    ) -> ExecutionResult {
        //self.schema.collections().find()
        //executor.resolve(collection, &Collection::new())
        unimplemented!()
    }

    fn resolve_collections(
        &self,
        info: &DbSchema,
        context: &Context<C, S>,
        _args: &Arguments,
        executor: &Executor<Context<C, S>>,
        coll_name: &str,
    ) -> ExecutionResult {
        Self::unwrap_collection(info, context, coll_name, |coll| {
            executor.resolve_with_ctx(
                &(format!("{}Connection", coll_name).as_str(), coll_name, info),
                &Connection::new(&coll),
            )
        })
    }

fn unwrap_collection<CB: FnOnce(&RwLockReadGuard<<<C as shelf_database::Cache>::CacheSchema as shelf_database::CacheSchema>::CacheCollection>) -> ExecutionResult>(info: &DbSchema, context: &Context<C, S>, coll_name: &str, callback: CB) -> ExecutionResult{
        let db = context.db.read().unwrap();
        match db.schema(&context.logger, info.id) {
            Some(lock) => {
                let schema = lock.read().unwrap();
                match schema.collection_by_name(coll_name) {
                    Some(coll_lock) => {
                        let coll = coll_lock.read().unwrap();
                        callback(&coll)
                    }
                    None => {
                        error!(context.logger, "Trying to ask for data from a collection that does not exist"; "schema_id" => info.id.to_string(), "collection_name" => coll_name);
                        Err(FieldError::new(
                            "Internal server error",
                            graphql_value!({ "internal_error": "The collection does not exists" }),
                        ))
                    }
                }
            }
            None => {
                error!(context.logger, "Trying to ask for data from a schema that does not exist"; "schema_name" => info.name.to_string(), "schema_id" => info.id.to_string());
                Err(FieldError::new(
                    "Internal server error",
                    graphql_value!({ "internal_error": "The schema does not exists" }),
                ))
            }
        }
    }
}

impl<'a, C: Cache, S: Store> GraphQLType for Query<C, S> {
    type Context = Context<C, S>;
    type TypeInfo = DbSchema;

    fn name(_info: &Self::TypeInfo) -> Option<&'static str> {
        Some("Query")
    }

    fn meta<'r>(
        info: &Self::TypeInfo,
        registry: &mut Registry<'r, DefaultScalarValue>,
    ) -> MetaType<'r, DefaultScalarValue>
    where
        DefaultScalarValue: 'r,
    {
        let collections = match info.types() {
            Some(data) => data
                .collections
                .iter()
                .map(|i| {
                    let fields = i.fields.iter().map(|f| f.name.to_string()).collect();
                    (i.name.to_string(), fields)
                })
                .collect(),
            None => vec![],
        };
        let fields = QueryField::fields::<C, S>(&info, registry, &collections);
        registry
            .build_object_type::<Query<C, S>>(&info, &fields)
            .into_meta()
    }
}

impl<C: Cache, S: Store> GraphQLTypeAsync<DefaultScalarValue> for Query<C, S> {
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        field_name: &'a str,
        arguments: &'a Arguments<DefaultScalarValue>,
        executor: &'a Executor<Self::Context, DefaultScalarValue>,
    ) -> BoxFuture<'a, ExecutionResult<DefaultScalarValue>> {
        async move {
            // Next, we need to match the queried field name. All arms of this
            // match statement return `ExecutionResult`, which makes it hard to
            // statically verify that the type you pass on to `executor.resolve*`
            // actually matches the one that you defined in `meta()` above.
            let context = executor.context();

            match QueryField::from_str(field_name).unwrap() {
                QueryField::Node => executor.resolve_with_ctx(&(), &Node::new()),
                QueryField::SchemaId => executor.resolve_with_ctx(&(), &info.id),
                QueryField::SchemaName => executor.resolve_with_ctx(&(), &info.name),
                QueryField::SchemaCreatedAt => executor.resolve_with_ctx(&(), &info.created_at),
                QueryField::Document { .. } => {
                    self.resolve_collection(context, arguments, executor)
                }
                QueryField::Documents { collection_name } => {
                    self.resolve_collections(info, context, arguments, executor, &collection_name)
                }
                QueryField::FirstDocumentByField {
                    collection_name: _,
                    field_name: _,
                } => unimplemented!(),
                QueryField::FindDocumentsByField {
                    collection_name: _,
                    field_name: _,
                } => unimplemented!(),
                QueryField::FirstDocumentByFieldAndField {
                    collection_name: _,
                    field_name: _,
                    second_field_name: _,
                } => unimplemented!(),
                QueryField::FindDocumentsByFieldAndField {
                    collection_name: _,
                    field_name: _,
                    second_field_name: _,
                } => unimplemented!(),
            }
        }
        .boxed()
    }
}
