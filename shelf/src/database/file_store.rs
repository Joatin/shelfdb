use crate::database::store::Store;
use failure::Error;
use slog::Logger;
use futures::{Future, FutureExt};
use std::pin::Pin;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::fs::create_dir;
use tokio::fs::OpenOptions;
use std::path::Path;
use crate::collection::{Schema, Collection};

pub struct FileStore {
    logger: Logger,
    base_path: String
}

impl FileStore {

    pub async fn new(logger: &Logger) -> Result<Self, Error> {
        info!(logger, "Setting up file store");

        Ok(Self {
            logger: logger.clone(),
            base_path: ".shelf_data".to_owned()
        })
    }
}

impl Store for FileStore {

    fn get_schemas(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>> {
        let logger = self.logger.clone();
        let base_path = self.base_path.clone();

        async move {
            let path = Path::new(&base_path).join("schemas.json");

            if path.is_file() {
                debug!(logger, "Fetching schemas from file");
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(path)
                    .await?;

                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;

                let result = serde_json::from_slice::<Vec<Schema>>(&contents)?;

                Ok(result)
            } else {
                warn!(logger, "No schemas file detected, creating new");
                if !Path::new(&base_path).is_dir() {
                    create_dir(&base_path).await?;
                }
                let mut file = File::create(path).await?;
                file.write_all(b"[]").await?;

                Ok(vec![])
            }
        }.boxed()
    }


    fn save_schema(&self, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let logger = self.logger.clone();
        let base_path = self.base_path.clone();
        let our_schema = schema.clone();
        let all_schemas_fut = self.get_schemas();

        async move {
            let mut schemas = all_schemas_fut.await?;
            let schema_name = our_schema.name.to_owned();
            let schema_id = our_schema.id.to_owned();

            schemas.push(our_schema);

            let path = Path::new(&base_path).join("schemas.json");
            let mut file = OpenOptions::new()
                .write(true)
                .open(path)
                .await?;

            let data = serde_json::to_string_pretty(&schemas)?;

            file.write_all(&data.as_bytes()).await?;

            create_dir(Path::new(&base_path).join(&schema_name)).await?;

            info!(logger, "Saved schema \"{}\"", schema_name; "id" => schema_id.to_string(), "name" => &schema_name);

            Ok(())
        }.boxed()
    }

    fn get_collections(&self, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<Vec<Collection>, Error>> + Send>> {
        let logger = self.logger.clone();
        let base_path = self.base_path.clone();
        let schema_name = schema.name.to_owned();

        async move  {
            let path = Path::new(&base_path).join(&schema_name).join("collections.json");

            if path.is_file() {
                debug!(logger, "Fetching collections from file");
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(path)
                    .await?;

                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;

                let result = serde_json::from_slice::<Vec<Collection>>(&contents)?;

                Ok(result)
            } else {
                warn!(logger, "No collections file detected, creating new");
                if !Path::new(&base_path).is_dir() {
                    create_dir(&base_path).await?;
                }
                let mut file = File::create(path).await?;
                file.write_all(b"[]").await?;

                Ok(vec![])
            }
        }.boxed()
    }

    fn save_collection(&self, schema: &Schema, collection: &Collection) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let logger = self.logger.clone();
        let base_path = self.base_path.clone();
        let schema_name = schema.name.to_owned();
        let schema_id = schema.id.to_owned();
        let our_collection = collection.clone();
        let all_collections_fut = self.get_collections(&schema);

        async move {
            let mut collections = all_collections_fut.await?;
            let collection_name = our_collection.name.to_owned();
            let collection_id = our_collection.id.to_owned();

            collections.push(our_collection);

            let path = Path::new(&base_path).join(&schema_name).join("collections.json");
            let mut file = OpenOptions::new()
                .write(true)
                .open(path)
                .await?;

            let data = serde_json::to_string_pretty(&collections)?;

            file.write_all(&data.as_bytes()).await?;

            let doc_name = format!("{}_docs", collection_name);
            let index_name = format!("{}_indexes", collection_name);

            create_dir(Path::new(&base_path).join(&schema_name).join(doc_name)).await?;
            create_dir(Path::new(&base_path).join(&schema_name).join(index_name)).await?;

            info!(logger, "Saved collection \"{}\"", collection_name; "schema_id" => schema_id.to_string(), "schema_name" => &schema_name, "id" => collection_id.to_string(), "name" => &collection_name);

            Ok(())
        }.boxed()
    }
}