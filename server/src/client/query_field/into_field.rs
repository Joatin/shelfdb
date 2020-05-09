use super::QueryField;
use crate::client::{
    collection::Collection,
    connection::Connection,
    node::Node,
};
use chrono::{
    DateTime,
    Utc,
};
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

impl QueryField {
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
