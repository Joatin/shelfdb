use crate::context::Context;
use juniper::{GraphQLType, DefaultScalarValue, Registry, Arguments, Executor, ExecutionResult, FieldError};
use juniper::meta::MetaType;
use shelf_database::{Schema as DbSchema, Store, Cache};
use inflector::cases::camelcase::to_camel_case;
use crate::client::connection::Connection;
use crate::client::collection::Collection;
use crate::client::node::Node;
use failure::_core::marker::PhantomData;
use shelf_database::CacheSchema;
use uuid::Uuid;
use std::sync::RwLockReadGuard;


pub struct Query<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Query<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData
        }
    }

    fn resolve_collection(&self, _context: &Context<C, S>, _args: &Arguments, _executor: &Executor<Context<C, S>>) -> ExecutionResult {
        //self.schema.collections().find()
        //executor.resolve(collection, &Collection::new())
        unimplemented!()
    }

    fn resolve_collections(&self, info: &DbSchema, context: &Context<C, S>, _args: &Arguments, executor: &Executor<Context<C, S>>, coll_name: &str) -> ExecutionResult {
        Self::unwrap_collection(info, context, coll_name, |coll| {
            executor.resolve_with_ctx(&(format!("{}Connection", coll_name).as_str(), coll_name, info), &Connection::new(&coll))
        })
    }

    fn unwrap_collection<CB: FnOnce(&RwLockReadGuard<<<C as shelf_database::Cache>::CacheSchema as shelf_database::CacheSchema>::CacheCollection>) -> ExecutionResult>(info: &DbSchema, context: &Context<C, S>, coll_name: &str, callback: CB) -> ExecutionResult {
        let db = context.db.read().unwrap();
        match db.schema(&context.logger, info.id) {
            Some(lock) => {
                let schema = lock.read().unwrap();
                match schema.collection_by_name(coll_name) {
                    Some(coll_lock) => {
                        let coll = coll_lock.read().unwrap();
                        callback(&coll)
                    },
                    None => {
                        error!(context.logger, "Trying to ask for data from a collection that does not exist"; "schema_id" => info.id.to_string(), "collection_name" => coll_name);
                        Err(FieldError::new(
                            "Internal server error",
                            graphql_value!({ "internal_error": "The collection does not exists" })
                        ))
                    }
                }
            },
            None => {
                error!(context.logger, "Trying to ask for data from a schema that does not exist"; "schema_name" => info.name.to_string(), "schema_id" => info.id.to_string());
                Err(FieldError::new(
                    "Internal server error",
                    graphql_value!({ "internal_error": "The schema does not exists" })
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

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, DefaultScalarValue>) -> MetaType<'r, DefaultScalarValue>
        where DefaultScalarValue: 'r
    {
        let mut fields = vec![
            registry.field::<&Node<C, S>>("node", &()),
            registry.field::<&Uuid>("schemaId", &()),
            registry.field::<&String>("schemaName", &()),
            registry.field::<&String>("schemaCreatedAt", &())
        ];

        if let Some(types) = info.types() {
            for coll in types.collections {
                fields.push(registry.field::<&i32>(&format!("{}Count", to_camel_case(&coll.name)), &()));
                fields.push(registry.field::<&Connection<C, S>>(&format!("{}s", to_camel_case(&coll.name)), &(&format!("{}Connection", coll.name), &coll.name, info)));
                fields.push(registry.field::<&Collection<C, S>>(&to_camel_case(&coll.name), &(&coll.name, info)));
            }
        }

        registry.build_object_type::<Query<C, S>>(&info, &fields).into_meta()
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        args: &Arguments,
        executor: &Executor<Self::Context>
    )
        -> ExecutionResult
    {
        // Next, we need to match the queried field name. All arms of this
        // match statement return `ExecutionResult`, which makes it hard to
        // statically verify that the type you pass on to `executor.resolve*`
        // actually matches the one that you defined in `meta()` above.
        let context = executor.context();

        match field_name {
            "node" => executor.resolve(&(), &Node::new()),
            "schemaId" => executor.resolve_with_ctx(&(), &info.id),
            "schemaName" => executor.resolve_with_ctx(&(), &info.name),
            "schemaCreatedAt" => executor.resolve_with_ctx(&(), &info.created_at),
            _ => {
                if let Some(types) = info.types() {
                    if let Some(coll) = types.collections.iter().find(|i| field_name.starts_with(&to_camel_case(&i.name))) {
                        let coll_name = to_camel_case(&coll.name);
                        if field_name == format!("{}", coll_name) {
                            self.resolve_collection(context, args, executor)
                        } else if field_name == format!("{}s", coll_name) {
                            self.resolve_collections(info, context, args, executor, &coll.name)
                        } else if field_name == format!("{}Count", coll_name) {
                            // executor.resolve_with_ctx(&(), &(collection.document_count() as i32))
                            unimplemented!()
                        } else {
                            panic!("Field {} not found", field_name)
                        }
                    } else {
                        panic!("Field {} not found", field_name)
                    }
                } else {
                    panic!("Field {} not found", field_name)
                }
            }
        }
    }
}


