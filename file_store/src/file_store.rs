use failure::Error;
use slog::Logger;
use futures::{Future, FutureExt};
use std::pin::Pin;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::fs::create_dir;
use tokio::fs::OpenOptions;
use std::path::Path;
use shelf_database::{Schema, Collection, Store};
use shelf_config::Config;

pub struct FileStore {
    logger: Logger,
    base_path: String
}

impl FileStore {

    pub async fn new(logger: &Logger, config: &Config) -> Result<Self, Error> {
        info!(logger, "Setting up file store");

        Ok(Self {
            logger: logger.clone(),
            base_path: config.data_folder.to_owned()
        })
    }

    async fn save_collections(logger: &Logger, base_path: &str, schema_name: &str, collections: &Vec<Collection>) -> Result<(), Error> {

        debug!(logger, "Saving collections for schema {}", schema_name);

        let path = Path::new(&base_path).join(&schema_name).join("collections.json");

        if !path.is_file() {
            File::create(path.clone()).await?;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .await?;

        let data = serde_json::to_string_pretty(&collections)?;

        file.write_all(&data.as_bytes()).await?;

        for collection in collections {
            let doc_name = format!("{}_docs", collection.name);
            let index_name = format!("{}_indexes", collection.name);

            create_dir(Path::new(&base_path).join(&schema_name).join(doc_name)).await?;
            create_dir(Path::new(&base_path).join(&schema_name).join(index_name)).await?;

            debug!(logger, "Saved collection \"{}\"", collection.name; "schema_name" => &schema_name, "id" => collection.id.to_string(), "name" => &collection.name);
        }

        Ok(())
    }


    async fn get_collections(logger: &Logger, base_path: &str, schema_name: &str) -> Result<Vec<Collection>, Error> {
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

                match serde_json::from_slice::<Vec<Schema>>(&contents) {
                    Ok(mut result) => {
                        for mut schema in &mut result {
                            let collections = Self::get_collections(&logger, &base_path, &schema.name).await?;
                            schema.collections = collections;
                        }

                        Ok(result)
                    },
                    Err(e) => {
                        crit!(logger, "Failed to parse schema file, perhaps the file has been corrupted?"; "error" => format!("{}", e));
                        Err(e.into())
                    },
                }
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
        let schema = schema.clone();
        let all_schemas_fut = self.get_schemas();

        async move {
            let mut schemas = all_schemas_fut.await?;
            let schema_name = schema.name.to_owned();
            let schema_id = schema.id.to_owned();
            let collections = schema.collections.clone();

            schemas.push(schema);

            let path = Path::new(&base_path).join("schemas.json");
            let mut file = OpenOptions::new()
                .write(true)
                .open(path)
                .await?;

            let data = serde_json::to_string_pretty(&schemas)?;

            file.write_all(&data.as_bytes()).await?;

            create_dir(Path::new(&base_path).join(&schema_name)).await?;

            debug!(logger, "Saved schema \"{}\"", schema_name; "id" => schema_id.to_string(), "name" => &schema_name);

            Self::save_collections(&logger, &base_path, &schema_name, &collections).await?;

            Ok(())
        }.boxed()
    }
}