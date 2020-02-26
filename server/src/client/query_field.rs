use crate::client::{
    collection::Collection,
    connection::Connection,
    node::Node,
};
use chrono::{
    DateTime,
    Utc,
};
use failure::Error;
use inflector::cases::{
    camelcase::to_camel_case,
    classcase::to_class_case,
};
use juniper::{
    meta::{
        Argument,
        DeprecationStatus,
        Field,
    },
    DefaultScalarValue,
    InputValue,
    Registry,
};
use shelf_database::{
    Cache,
    Schema as DbSchema,
    Store,
};
use uuid::Uuid;

pub enum QueryField {
    Node,
    SchemaId,
    SchemaName,
    SchemaCreatedAt,
    Document {
        collection_name: String,
    },
    Documents {
        collection_name: String,
    },
    FirstDocumentByField {
        collection_name: String,
        field_name: String,
    },
    FindDocumentsByField {
        collection_name: String,
        field_name: String,
    },
    FirstDocumentByFieldAndField {
        collection_name: String,
        field_name: String,
        second_field_name: String,
    },
    FindDocumentsByFieldAndField {
        collection_name: String,
        field_name: String,
        second_field_name: String,
    },
}

impl QueryField {
    pub fn from_str(
        field_name: &str,
        collections: &[(String, Vec<String>)],
    ) -> Result<QueryField, Error> {
        match field_name {
            "node" => Ok(QueryField::Node),
            "schemaId" => Ok(QueryField::SchemaId),
            "schemaName" => Ok(QueryField::SchemaName),
            "schemaCreatedAt" => Ok(QueryField::SchemaCreatedAt),
            _ => {
                if let Some((name, _)) = collections
                    .iter()
                    .find(|(name, _fields)| field_name == to_camel_case(name))
                {
                    Ok(QueryField::Document {
                        collection_name: name.to_string(),
                    })
                } else if let Some((name, _)) = collections
                    .iter()
                    .find(|(name, _fields)| field_name == format!("{}s", to_camel_case(name)))
                {
                    Ok(QueryField::Documents {
                        collection_name: name.to_string(),
                    })
                } else {
                    bail!("Unknown field")
                }
            }
        }
    }

    pub fn fields<'r, C: Cache, S: Store>(
        info: &DbSchema,
        registry: &mut Registry<'r, DefaultScalarValue>,
        collections: &[(String, Vec<String>)],
    ) -> Vec<Field<'r, DefaultScalarValue>> {
        let mut fields = vec![
            QueryField::Node.into_field::<C, S>(info, registry),
            QueryField::SchemaId.into_field::<C, S>(info, registry),
            QueryField::SchemaName.into_field::<C, S>(info, registry),
            QueryField::SchemaCreatedAt.into_field::<C, S>(info, registry),
        ];

        for (collection_name, collection_fields) in collections {
            fields.push(
                QueryField::Document {
                    collection_name: collection_name.to_string(),
                }
                .into_field::<C, S>(info, registry),
            );
            fields.push(
                QueryField::Documents {
                    collection_name: collection_name.to_string(),
                }
                .into_field::<C, S>(info, registry),
            );

            for field in collection_fields
                .iter()
                .filter(|i| !i.eq(&&"id".to_string()))
            {
                fields.push(
                    QueryField::FirstDocumentByField {
                        collection_name: collection_name.to_string(),
                        field_name: field.to_string(),
                    }
                    .into_field::<C, S>(info, registry),
                );
                fields.push(
                    QueryField::FindDocumentsByField {
                        collection_name: collection_name.to_string(),
                        field_name: field.to_string(),
                    }
                    .into_field::<C, S>(info, registry),
                );

                for second_field in collection_fields
                    .iter()
                    .filter(|i| !i.eq(&&"id".to_string()) && !i.eq(&&field.to_string()))
                {
                    fields.push(
                        QueryField::FirstDocumentByFieldAndField {
                            collection_name: collection_name.to_string(),
                            field_name: field.to_string(),
                            second_field_name: second_field.to_string(),
                        }
                        .into_field::<C, S>(info, registry),
                    );
                    fields.push(
                        QueryField::FindDocumentsByFieldAndField {
                            collection_name: collection_name.to_string(),
                            field_name: field.to_string(),
                            second_field_name: second_field.to_string(),
                        }
                        .into_field::<C, S>(info, registry),
                    );
                }
            }
        }

        fields
    }

    // fields.push(registry.field::<&i32>(&format!("{}Count",
    // to_camel_case(&coll.name)), &())); fields.push(registry.field::<&
    // Connection<C, S>>(&format!("{}s", to_camel_case(&coll.name)),
    // &(&format!("{}Connection", coll.name), &coll.name, info)));
    // fields.push(registry.field::<&Collection<C, S>>(&to_camel_case(&coll.name),
    // &(&coll.name, info)));

    pub fn into_field<'r, C: Cache, S: Store>(
        self,
        info: &DbSchema,
        registry: &mut Registry<'r, DefaultScalarValue>,
    ) -> Field<'r, DefaultScalarValue> {
        match self {
            QueryField::Node => {
                Field {
                    name: "node".to_string(),
                    description: Some("\"Retrieves a document by it's id. This field ensures compatibility with relay\"".to_string()),
                    arguments: Some(vec![
                        Argument {
                            name: "id".to_string(),
                            description: Some("\"The id of the document you wish to retrieve, has to be a valid Uuid\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Option<Node<C, S>>>(&()),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::SchemaId => {
                Field {
                    name: "schemaId".to_string(),
                    description: Some("\"Returns the id of the schema you are currently accessing. Exists for convenience\"".to_string()),
                    arguments: None,
                    field_type: registry.get_type::<Uuid>(&()),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::SchemaName => {
                Field {
                    name: "schemaName".to_string(),
                    description: Some("\"Returns the name of the schema you are currently accessing. Exists for convenience\"".to_string()),
                    arguments: None,
                    field_type: registry.get_type::<String>(&()),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::SchemaCreatedAt => {
                Field {
                    name: "schemaCreatedAt".to_string(),
                    description: Some("\"Returns the date the schema you are currently accessing was created. Exists for convenience\"".to_string()),
                    arguments: None,
                    field_type: registry.get_type::<DateTime<Utc>>(&()),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::Document { collection_name } => {
                Field {
                    name: to_camel_case(&collection_name),
                    description: Some(format!("\"Returns a single document from the {} collection. You have to provide the Uuid of the document you want to find. If the document does not exist in the database, then null will be returned\"", collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: "id".to_string(),
                            description: Some("\"The id of the document you wish to retrieve, has to be a valid Uuid\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Option<Collection<C, S>>>(&(collection_name.to_string(), info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::Documents { collection_name } => {
                Field {
                    name: format!("{}s", to_camel_case(&collection_name)),
                    description: Some(format!("\"This gives back a connection of documents from the {} collection. You can use the connection to get info about the next and previous pages, as well as the total count of documents\"", collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: "first".to_string(),
                            description: Some("\"The number of documents to return\"".to_string()),
                            arg_type: registry.get_type::<Option<i32>>(&()),
                            default_value: Some(InputValue::Scalar(DefaultScalarValue::Int(50)))
                        },
                        Argument {
                            name: "after".to_string(),
                            description: Some("\"Return documents after this cursor, can not be used together with before\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        },
                        Argument {
                            name: "before".to_string(),
                            description: Some("\"Return documents before this cursor, can not be used together with after\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Connection<C, S>>(&(format!("{}Connection", collection_name), collection_name, info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::FirstDocumentByField { collection_name, field_name } => {
                Field {
                    name: format!("first{}By{}", to_class_case(&collection_name), to_class_case(&field_name)),
                    description: Some(format!("\"Returns the first document matching {} in collection {}\"", field_name, collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: field_name,
                            description: Some("\"The value you want to match field with\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Option<Collection<C, S>>>(&(collection_name, info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::FindDocumentsByField { collection_name, field_name } => {
                Field {
                    name: format!("find{}sBy{}", to_class_case(&collection_name), to_class_case(&field_name)),
                    description: Some(format!("\"This gives back a connection of documents that matches {} from the {} collection. You can use the connection to get info about the next and previous pages, as well as the total count of documents\"", field_name, collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: field_name,
                            description: Some("\"The value of the field to find\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        },
                        Argument {
                            name: "first".to_string(),
                            description: Some("\"The number of documents to return\"".to_string()),
                            arg_type: registry.get_type::<Option<i32>>(&()),
                            default_value: Some(InputValue::Scalar(DefaultScalarValue::Int(50)))
                        },
                        Argument {
                            name: "after".to_string(),
                            description: Some("\"Return documents after this cursor, can not be used together with before\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        },
                        Argument {
                            name: "before".to_string(),
                            description: Some("\"Return documents before this cursor, can not be used together with after\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Connection<C, S>>(&(format!("{}Connection", collection_name), collection_name, info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::FirstDocumentByFieldAndField { collection_name, field_name, second_field_name } => {
                Field {
                    name: format!("first{}By{}And{}", to_class_case(&collection_name), to_class_case(&field_name), to_class_case(&second_field_name)),
                    description: Some(format!("\"Returns the first document matching {} and {} in collection {}\"", field_name, second_field_name, collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: field_name,
                            description: Some("\"The value you want to match field with\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        },
                        Argument {
                            name: second_field_name,
                            description: Some("\"The value of the second field you want to match field with\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Option<Collection<C, S>>>(&(collection_name, info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
            QueryField::FindDocumentsByFieldAndField { collection_name, field_name, second_field_name } => {
                Field {
                    name: format!("find{}sBy{}And{}", to_class_case(&collection_name), to_class_case(&field_name), to_class_case(&second_field_name)),
                    description: Some(format!("\"This gives back a connection of documents that matches {} and {} from the {} collection. You can use the connection to get info about the next and previous pages, as well as the total count of documents\"", field_name, second_field_name, collection_name)),
                    arguments: Some(vec![
                        Argument {
                            name: field_name,
                            description: Some("\"The value of the field to find\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        },
                        Argument {
                            name: second_field_name,
                            description: Some("\"The value of the second field to find\"".to_string()),
                            arg_type: registry.get_type::<String>(&()),
                            default_value: None
                        },
                        Argument {
                            name: "first".to_string(),
                            description: Some("\"The number of documents to return\"".to_string()),
                            arg_type: registry.get_type::<Option<i32>>(&()),
                            default_value: Some(InputValue::Scalar(DefaultScalarValue::Int(50)))
                        },
                        Argument {
                            name: "after".to_string(),
                            description: Some("\"Return documents after this cursor, can not be used together with before\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        },
                        Argument {
                            name: "before".to_string(),
                            description: Some("\"Return documents before this cursor, can not be used together with after\"".to_string()),
                            arg_type: registry.get_type::<Option<String>>(&()),
                            default_value: None
                        }
                    ]),
                    field_type: registry.get_type::<Connection<C, S>>(&(format!("{}Connection", collection_name), collection_name, info.clone())),
                    deprecation_status: DeprecationStatus::Current
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::client::query_field::QueryField;
    use fnv::{
        FnvBuildHasher,
        FnvHashMap,
    };
    use juniper::{
        DefaultScalarValue,
        Registry,
    };
    use shelf_database::{
        test::{
            TestCache,
            TestStore,
        },
        Schema as DbSchema,
    };
    use uuid::Uuid;

    fn registry<'r>() -> Registry<'r, DefaultScalarValue> {
        Registry::new(FnvHashMap::with_hasher(FnvBuildHasher::default()))
    }

    fn schema() -> DbSchema {
        DbSchema::new(Uuid::nil(), "TEST", None)
    }

    #[test]
    fn fields_should_contain_schema_id() {
        let mut registry = registry();
        let fields = QueryField::fields::<TestCache, TestStore>(&schema(), &mut registry, &[]);

        assert!(
            fields.iter().any(|i| i.name == "schemaId"),
            "The fields did not contain schemaId"
        );
    }

    #[test]
    fn fields_should_contain_schema_name() {
        let mut registry = registry();
        let fields = QueryField::fields::<TestCache, TestStore>(&schema(), &mut registry, &[]);

        assert!(
            fields.iter().any(|i| i.name == "schemaName"),
            "The fields did not contain schemaName"
        );
    }

    #[test]
    fn fields_should_contain_schema_created_at() {
        let mut registry = registry();
        let fields = QueryField::fields::<TestCache, TestStore>(&schema(), &mut registry, &[]);

        assert!(
            fields.iter().any(|i| i.name == "schemaCreatedAt"),
            "The fields did not contain schemaCreatedAt"
        );
    }
}
