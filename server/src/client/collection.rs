use crate::context::Context;
use graphql_parser::schema::Type as GType;
use juniper::{
    meta::{
        DeprecationStatus,
        Field,
        MetaType,
    },
    to_camel_case,
    Arguments,
    DefaultScalarValue,
    ExecutionResult,
    Executor,
    FieldError,
    GraphQLType,
    Registry,
    Type,
};
use serde_json::Value;
use shelf_database::{
    Cache,
    Document,
    Schema as DbSchema,
    Store,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
    sync::Arc,
};
use uuid::Uuid;

pub struct Collection<C: Cache, S: Store> {
    document: Arc<Document>,
    phantom_store: PhantomData<S>,
    phantom_cache: PhantomData<C>,
}

impl<C: Cache, S: Store> Collection<C, S> {
    pub fn new(document: Arc<Document>) -> Self {
        Self {
            document,
            phantom_store: PhantomData,
            phantom_cache: PhantomData,
        }
    }
}

impl<C: Cache, S: Store> GraphQLType for Collection<C, S> {
    type Context = Context<C, S>;
    type TypeInfo = (String, DbSchema);

    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.0)
    }

    fn meta<'r>(
        info: &Self::TypeInfo,
        mut registry: &mut Registry<'r, DefaultScalarValue>,
    ) -> MetaType<'r, DefaultScalarValue>
    where
        DefaultScalarValue: 'r,
    {
        let mut fields = vec![];

        if let Some(types) = info.1.types() {
            if let Some(coll) = types.collections.iter().find(|i| i.name == info.0) {
                for field in &coll.fields {
                    fields.push(Field {
                        name: field.name.to_owned(),
                        description: field.description.as_ref().map(|f| format!("\"{}\"", f)),
                        arguments: None,
                        field_type: get_field_type(&mut registry, &field.field_type),
                        deprecation_status: DeprecationStatus::Current,
                    })
                }
            }
        }

        let meta_object = registry
            .build_object_type::<Collection<C, S>>(&info, &fields)
            .into_meta();

        if let MetaType::Object(m) = meta_object {
            MetaType::Object(m.interfaces(&[Type::Named(Cow::from("Node"))]))
        } else {
            meta_object
        }
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _args: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        if field_name == "id" {
            return executor.resolve_with_ctx(&(), &self.document.id);
        }

        if let Some(types) = info.1.types() {
            if let Some(coll) = types.collections.iter().find(|i| i.name == info.0) {
                if let Some(field) = coll
                    .fields
                    .iter()
                    .find(|i| field_name == to_camel_case(&i.name))
                {
                    return match self.document.fields.get(&field.name) {
                        None => {
                            if let graphql_parser::schema::Type::NonNullType(_) = field.field_type {
                                error!(executor.context().logger, "The field was missing in the db, even though it was required...");
                                return Err(FieldError::new(
                                    "Missing field",
                                    graphql_value!({ "internal_error": "The field was missing in the db, even though it was required..." }),
                                ));
                            }
                            executor.resolve_with_ctx(&(), &Option::<String>::None)
                        }
                        Some(value) => match value {
                            Value::Null => executor.resolve_with_ctx(&(), &Option::<String>::None),
                            Value::Bool(v) => executor.resolve_with_ctx(&(), &v),
                            Value::Number(v) => {
                                executor.resolve_with_ctx(&(), &(v.as_i64().unwrap() as i32))
                            }
                            Value::String(v) => executor.resolve_with_ctx(&(), &v),
                            Value::Array(_) => unimplemented!(),
                            Value::Object(_) => unimplemented!(),
                        },
                    };
                }
            }
        }

        panic!("Field {} not found on type Collection", field_name)
    }
}

fn get_field_type<'r>(
    mut registry: &mut Registry<'r, DefaultScalarValue>,
    field: &GType,
) -> Type<'r> {
    match field {
        GType::NamedType(t) => match t.as_str() {
            "String" => registry.get_type::<String>(&()),
            "Uuid" => registry.get_type::<Uuid>(&()),
            "i32" => registry.get_type::<i32>(&()),
            _ => panic!("Unknown type!"),
        },
        GType::ListType(_) => panic!("Can't handle list types yet"),
        GType::NonNullType(nt) => match &**nt {
            GType::NamedType(_t) => get_field_type(&mut registry, nt),
            GType::ListType(_) => panic!("Can't handle list types yet"),
            GType::NonNullType(_) => panic!("Cant be doubly wrapped in non null"),
        },
    }
}
