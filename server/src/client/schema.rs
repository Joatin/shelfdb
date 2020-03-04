use crate::client::{
    mutation::Mutation,
    query::Query,
};
use juniper::RootNode;

pub type Schema<'a, C, S> = RootNode<'a, Query<C, S>, Mutation<C, S>>;

#[cfg(test)]
mod tests {
    use crate::{
        client::{
            mutation::Mutation,
            query::Query,
            Schema,
        },
        context::Context,
    };
    use juniper::{http::{
        GraphQLRequest,
        GraphQLResponse,
    }, DefaultScalarValue};
    use serde_json::{
        Map,
        Value,
    };
    use shelf_config::Config;
    use shelf_database::{
        test::TestStore,
        Cache,
        CacheCollection,
        CacheSchema,
        Database,
        Document,
        Schema as DbSchema,
    };
    use shelf_memory_cache::MemoryCache;
    use slog::Logger;
    use sloggers::{
        null::NullLoggerBuilder,
        Build,
    };
    use std::{
        collections::HashMap,
        sync::Arc,
    };
    use uuid::Uuid;

    const TEST_GRAPHQL_SCHEMA: &str = r#"
        directive @collection on OBJECT

        scalar Uuid

        type Car @collection {
            id: Uuid!
            brand: String!
            model: String!
        }
    "#;

    #[tokio::test]
    async fn get_schema_id() {
        let (root_node, context) = node_and_context().await;
        let request =
            GraphQLRequest::<DefaultScalarValue>::new("{schemaId}".to_string(), None, None);

        let response = request.execute_async(&root_node, &context).await;
        let data = unwrap_data_tag(response);
        let schema_id =
            serde_json::from_value::<Uuid>(data.get("schemaId").unwrap().clone()).unwrap();

        assert_eq!(schema_id, Uuid::nil(), "Got wrong id")
    }

    #[tokio::test]
    async fn get_schema_name() {
        let (root_node, context) = node_and_context().await;
        let request =
            GraphQLRequest::<DefaultScalarValue>::new("{schemaName}".to_string(), None, None);

        let response = request.execute_async(&root_node, &context).await;
        let data = unwrap_data_tag(response);
        let schema_name = data.get("schemaName").unwrap().as_str().unwrap();

        assert_eq!(schema_name, "Test", "Got wrong name")
    }

    #[tokio::test]
    #[ignore]
    async fn get_car_count() {
        let (root_node, context) = node_and_context().await;
        let request = GraphQLRequest::<DefaultScalarValue>::new(
            "{cars {totalCount}}".to_string(),
            None,
            None,
        );

        let response = request.execute_async(&root_node, &context).await;
        let data = unwrap_data_tag(response);
        let total_count = data
            .get("cars")
            .unwrap()
            .get("totalCount")
            .unwrap()
            .as_i64()
            .unwrap();

        assert_eq!(total_count, 1, "Got wrong count")
    }

    #[tokio::test]
    async fn get_car_by_id() {
        let (root_node, context) = node_and_context().await;
        let request = GraphQLRequest::<DefaultScalarValue>::new(
            format!("query {{car(id: \"{}\") {{id, model, brand}}}}", Uuid::nil()),
            None,
            None,
        );

        let response = request.execute_async(&root_node, &context).await;
        let data = unwrap_data_tag(response);
        let id = serde_json::from_value::<Uuid>(
            data
            .get("car")
            .unwrap()
            .get("id")
            .unwrap()
                .clone()
        ).unwrap();
        let model = data
            .get("car")
            .unwrap()
            .get("model")
            .unwrap()
            .as_str()
            .unwrap();
        let brand = data
            .get("car")
            .unwrap()
            .get("brand")
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(id, Uuid::nil(), "Got wrong id");
        assert_eq!(model, "Model S", "Got wrong model");
        assert_eq!(brand, "Tesla", "Got wrong brand");
    }

    fn unwrap_data_tag(response: GraphQLResponse<DefaultScalarValue>) -> Map<String, Value> {
        if response.is_ok() {
            let result = serde_json::to_value(response).unwrap();
            result.get("data").unwrap().as_object().unwrap().clone()
        } else {
            panic!(format!("Request was unsuccessful, error was: {}", serde_json::to_string(&response).unwrap()));
        }
    }

    async fn node_and_context<'a>() -> (
        Schema<'a, MemoryCache, TestStore>,
        Context<MemoryCache, TestStore>,
    ) {
        let logger = NullLoggerBuilder.build().unwrap();

        let db = database(&logger).await;
        let node = root_node(&db).await;
        let context = context(&logger, &db);

        (node, context)
    }

    async fn root_node<'a>(
        db: &Database<MemoryCache, TestStore>,
    ) -> Schema<'a, MemoryCache, TestStore> {
        let schema = db.cache().schema(Uuid::nil()).await.unwrap();
        let inner_schema = schema.inner_schema().await;
        Schema::new_with_info(
            Query::new(schema),
            Mutation::new(),
            inner_schema,
            (),
        )
    }

    fn context(
        logger: &Logger,
        db: &Database<MemoryCache, TestStore>,
    ) -> Context<MemoryCache, TestStore> {
        Context::new(&logger, Arc::new(Database::clone(&db)))
    }

    async fn database(logger: &Logger) -> Database<MemoryCache, TestStore> {
        let config = Config::default();
        let db = Database::new(
            &logger,
            &config,
            TestStore,
            MemoryCache::new(&logger).await.unwrap(),
        )
        .await
        .unwrap();

        db.cache()
            .insert_schema(
                &logger,
                DbSchema::new(Uuid::nil(), "Test", None),
                TEST_GRAPHQL_SCHEMA,
            )
            .await
            .unwrap();

        let mut fields = HashMap::new();

        fields.insert("brand".to_string(), "Tesla".into());
        fields.insert("model".to_string(), "Model S".into());

        let doc = Document {
            id: Uuid::nil(),
            fields,
        };

        db.cache()
            .schema(Uuid::nil())
            .await
            .unwrap()
            .collection_by_name("Car")
            .await
            .unwrap()
            .set_document(doc)
            .await;

        db
    }
}
