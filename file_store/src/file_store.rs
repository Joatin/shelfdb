use crate::util::{
    compute_hash_sum,
    extract_file_name,
    is_collection_file,
    write_compressed_file,
};
use colored::*;
use failure::Error;
use flate2::read::GzDecoder;
use futures::{
    future::{
        join_all,
        BoxFuture,
    },
    lock::Mutex,
    stream::StreamExt,
    Future,
    FutureExt,
};
use shelf_config::Config;
use shelf_database::{
    Collection,
    Document,
    Schema,
    Store,
};
use slog::Logger;
use std::{
    collections::HashMap,
    io::Read,
    mem,
    path::Path,
    pin::Pin,
    sync::Arc,
    time::Instant,
};
use tokio::{
    fs::{
        create_dir,
        read_dir,
        remove_file,
        rename,
        File,
        OpenOptions,
    },
    prelude::*,
    task,
};
use uuid::Uuid;

pub struct FileStore {
    base_path: String,
    collections: Mutex<HashMap<String, Vec<Collection>>>,
    documents: Mutex<HashMap<String, HashMap<String, Vec<Arc<Document>>>>>,
}

impl FileStore {
    pub async fn new(logger: &Logger, config: &Config) -> Result<Self, Error> {
        let b_path = Path::new(&config.data_folder);
        if !b_path.is_dir() {
            std::fs::create_dir(b_path)?;
        }

        info!(logger, "Setting up file store"; "data_folder" => Path::new(&config.data_folder).canonicalize().unwrap().to_str().unwrap().to_owned());

        Ok(Self {
            base_path: config.data_folder.to_owned(),
            collections: Mutex::new(HashMap::new()),
            documents: Mutex::new(HashMap::new()),
        })
    }

    async fn do_save_collections(&self, logger: &Logger) -> Result<(), Error> {
        let schemas = mem::replace(&mut *self.collections.lock().await, HashMap::new());
        for (schema_name, collections) in schemas {
            debug!(logger, "Saving collections for schema {}", schema_name);
            let base_path = Path::new(&self.base_path).join(schema_name);

            if !base_path.is_dir() {
                create_dir(base_path.clone()).await?;
            }

            let path = base_path.join("collections.json");

            debug!(logger, "Writing file");
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .await?;

            let data = serde_json::to_string_pretty(&collections).unwrap();

            file.write_all(&data.as_bytes()).await?;
        }
        debug!(logger, "Saved collections");

        Ok(())
    }

    async fn do_save_documents(&self, logger: &Logger) -> Result<(), Error> {
        let logger = logger.clone();
        let documents = mem::replace(&mut *self.documents.lock().await, HashMap::new());
        for (schema_name, collections) in documents {
            for (collection_name, documents) in collections {
                info!(
                    logger,
                    "Saving documents for collection {} in schema {}",
                    collection_name.yellow(),
                    &schema_name.yellow()
                );
                let base_path = Path::new(&self.base_path)
                    .join(&schema_name)
                    .join(format!("{}_docs", collection_name));
                if !base_path.is_dir() {
                    create_dir(base_path.clone()).await?;
                }

                let chunks = documents.chunks(10_000);

                let dir: Vec<_> = read_dir(base_path.clone()).await?.collect().await;

                for (index, chunk) in chunks.enumerate() {
                    if let Some(dir) = dir.iter().find(|i| {
                        i.as_ref()
                            .unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(&format!("{}_", index))
                    }) {
                        let (file_name, hash) = extract_file_name(&dir)?;
                        if !is_collection_file(&file_name) {
                            continue;
                        }
                        let data = serde_json::to_string(&chunk).unwrap();
                        let sum = compute_hash_sum(&data);

                        if sum == hash {
                            debug!(
                                logger,
                                "Chunk {} has not changed for collection {} in schema {}",
                                index,
                                collection_name,
                                &schema_name
                            );
                        } else {
                            let new_path = format!("{}~old", file_name);
                            if rename(base_path.join(&file_name), base_path.join(&new_path))
                                .await
                                .is_ok()
                            {
                                debug!(
                                    logger,
                                    "Renamed old collection file in case all goes wrong"
                                );
                            } else {
                                error!(logger, "Failed to rename old collection file"; "file_name" => &file_name);
                                bail!("Failed to rename old collection file");
                            }

                            let path = base_path.join(&format!("{}_{}.gz", index, sum));
                            write_compressed_file(&data, &path).await?;

                            if remove_file(base_path.join(&new_path)).await.is_ok() {
                                debug!(logger, "Removed old collection file");
                            }
                        }
                    } else {
                        let data = serde_json::to_string(&chunk).unwrap();
                        let path =
                            base_path.join(&format!("{}_{}.gz", index, compute_hash_sum(&data)));
                        write_compressed_file(&data, &path).await?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Store for FileStore {
    fn get_schemas<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<HashMap<Uuid, Schema>, Error>> {
        let logger = logger.clone();
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

                match serde_json::from_slice::<HashMap<Uuid, Schema>>(&contents) {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        crit!(logger, "Failed to parse schema file, perhaps the file has been corrupted?"; "error" => format!("{}", e));
                        Err(e.into())
                    },
                }
            } else {
                warn!(logger, "No schemas file detected");
                Ok(HashMap::new())
            }
        }.boxed()
    }

    fn get_collections(
        &self,
        logger: &Logger,
        schema: &Schema,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Collection>, Error>> + Send>> {
        let logger = logger.new(
            o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string()),
        );
        let path = Path::new(&self.base_path)
            .join(&schema.name)
            .join("collections.json");
        let base_path = self.base_path.to_string();

        async move {
            if path.is_file() {
                debug!(logger, "Fetching collections from file");
                let mut file = OpenOptions::new().read(true).open(path).await?;

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
        .boxed()
    }

    fn get_documents(
        &self,
        logger: &Logger,
        schema: &Schema,
        collection: &Collection,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Document>, Error>> + Send>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string(), "collection_name" => collection.name.to_string(), "collection_id" => collection.id.to_string()));
        let path = Path::new(&self.base_path)
            .join(&schema.name)
            .join(&format!("{}_docs", collection.name));

        async move {
            let mut documents = vec![];

            if path.is_dir() {
                let files: Vec<_> = path.read_dir().unwrap().collect();

                let contents = join_all(files.into_iter().map(|file| async {
                    if let Ok(dir) = file {
                        let name = dir.file_name().to_str().unwrap().to_string();
                        trace!(logger, "Reading file {}", name.yellow());
                        let mut file = OpenOptions::new()
                            .read(true)
                            .open(path.join(name.clone()))
                            .await?;

                        let mut contents = vec![];
                        file.read_to_end(&mut contents).await?;

                        let res = task::spawn_blocking(move || {
                            let mut gz = GzDecoder::new(&contents[..]);
                            let mut decoded = String::new();
                            gz.read_to_string(&mut decoded).unwrap();

                            let result = serde_json::from_str::<Vec<Document>>(&decoded)?;

                            Result::<Vec<Document>, Error>::Ok(result)
                        }).await??;

                        if res.is_empty() {
                            warn!(logger, "Weird, found file with no content"; "file_name" => &name);
                        }
                        trace!(logger, "Found {} documents in file {}", res.len().to_string().yellow(), name.yellow());

                        Ok(res)
                    } else {
                        bail!("Could not read file");
                    }
                }).collect::<Vec<_>>()).await;

                for content in contents {
                    if let Ok(mut data) = content {
                        documents.append(&mut data);
                    }
                }
            }

            if documents.is_empty() {
                warn!(logger, "No documents found");
            }

            Ok(documents)
        }.boxed()
    }

    fn save_schema<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
    ) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            let logger = logger.new(
                o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string()),
            );
            let mut schemas = self.get_schemas(&logger).await?;
            let schema_name = schema.name.to_owned();
            let schema_id = schema.id.to_owned();

            let base_path = Path::new(&self.base_path);

            schemas.insert(schema_id, schema.clone());

            if !base_path.is_dir() {
                info!(logger, "Root data folder is missing, creating it...");
                create_dir(base_path).await?;
            }

            let path = base_path.join("schemas.json");

            if rename(path.clone(), base_path.join("schemas.json~old"))
                .await
                .is_ok()
            {
                debug!(logger, "Renamed old schema file in case all goes wrong");
            }

            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .await?;

            debug!(logger, "Opened handle to new schema file");

            let data = serde_json::to_string_pretty(&schemas)?;

            trace!(
                logger,
                "New data to save in schema file is: {}",
                serde_json::to_string(&schemas).unwrap()
            );

            file.write_all(&data.as_bytes()).await?;

            debug!(logger, "Successfully saved schema {}", &schema_name);

            if remove_file(base_path.join("schemas.json~old"))
                .await
                .is_ok()
            {
                debug!(logger, "Removed old schema file");
            }

            if create_dir(base_path.join(&schema_name)).await.is_ok() {
                info!(logger, "Created new home dir for all collections");
            }

            debug!(logger, "Saved schema \"{}\"", schema_name);

            Ok(())
        }
        .boxed()
    }

    fn save_collection<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string(), "collection_name" => collection.name.to_string(), "collection_id" => collection.id.to_string()));
        debug!(logger, "Saving collection for schema {}", schema.name);
        async move {
            let mut collections = self.collections.lock().await;

            if let Some(list) = collections.get_mut(&schema.name) {
                list.push(collection.clone());
            } else {
                collections.insert(schema.name.to_string(), vec![collection.clone()]);
            }
            Ok(())
        }
        .boxed()
    }

    fn save_document<'a>(
        &'a self,
        _logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
        document: Arc<Document>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        async move {
            let mut documents = self.documents.lock().await;

            if let Some(schemas) = documents.get_mut(&schema.name) {
                if let Some(collections) = schemas.get_mut(&collection.name) {
                    collections.push(document);
                } else {
                    schemas.insert(collection.name.to_string(), vec![document]);
                }
            } else {
                let mut schemas = HashMap::new();
                schemas.insert(collection.name.to_string(), vec![document]);
                documents.insert(schema.name.to_string(), schemas);
            }
            Ok(())
        }
        .boxed()
    }

    fn flush<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        async move {
            let start_time = Instant::now();
            self.do_save_collections(&logger).await?;
            self.do_save_documents(&logger).await?;
            info!(logger, "\u{1f4bf} Saved data to disk"; "save_time" => format!("{:#?}", Instant::now().duration_since(start_time)));
            Ok(())
        }
        .boxed()
    }
}
