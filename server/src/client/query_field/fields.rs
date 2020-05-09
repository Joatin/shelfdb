use super::QueryField;
use juniper::{
    meta::Field,
    DefaultScalarValue,
    Registry,
};
use shelf_database::{
    Cache,
    Schema as DbSchema,
    Store,
};

impl QueryField {
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
