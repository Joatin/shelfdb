use failure::Error;
use slog::Logger;
use futures::{Future, FutureExt};
use std::pin::Pin;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::fs::create_dir;
use tokio::fs::read_dir;
use tokio::fs::OpenOptions;
use std::path::Path;
use shelf_database::{Schema, Collection, Store, Document};
use shelf_config::Config;
use std::collections::HashMap;
use futures::lock::Mutex;
use std::mem;
use futures::stream::StreamExt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Write, Read};
use flate2::read::GzDecoder;
use colored::*;

pub struct FileStore {
    base_path: String,
    collections: Mutex<HashMap<String, Vec<Collection>>>,
    documents: Mutex<HashMap<String, HashMap<String, Vec<Document>>>>,
}

impl FileStore {

    pub async fn new(logger: &Logger, config: &Config) -> Result<Self, Error> {
        info!(logger, "Setting up file store"; "data_folder" => Path::new(&config.data_folder).canonicalize().unwrap().to_str().unwrap().to_owned());

        Ok(Self {
            base_path: config.data_folder.to_owned(),
            collections: Mutex::new(HashMap::new()),
            documents: Mutex::new(HashMap::new()),
        })
    }
}

impl Store for FileStore {

    fn get_schemas(&self, logger: &Logger) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>> {
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

                match serde_json::from_slice::<Vec<Schema>>(&contents) {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        crit!(logger, "Failed to parse schema file, perhaps the file has been corrupted?"; "error" => format!("{}", e));
                        Err(e.into())
                    },
                }
            } else {
                warn!(logger, "No schemas file detected");
                Ok(vec![])
            }
        }.boxed()
    }


    fn get_collections(&self, logger: &Logger, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<Vec<Collection>, Error>> + Send>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string()));
        let path = Path::new(&self.base_path).join(&schema.name).join("collections.json");
        let base_path = self.base_path.to_string();

        async move {
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

    fn get_documents(&self, logger: &Logger, schema: &Schema, collection: &Collection) -> Pin<Box<dyn Future<Output=Result<Vec<Document>, Error>> + Send>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string(), "collection_name" => collection.name.to_string(), "collection_id" => collection.id.to_string()));
        let path = Path::new(&self.base_path).join(&schema.name).join(&format!("{}_docs", collection.name));

        async move {
            let mut documents = vec![];

            if path.is_dir() {
                let files: Vec<_> = path.read_dir().unwrap().collect();
                for file in files {
                    if let Ok(dir) = file {
                        let name = dir.file_name();
                        trace!(logger, "Reading file {}", name.to_str().unwrap().yellow());
                        let mut file = OpenOptions::new()
                            .read(true)
                            .open(path.join(name.clone()))
                            .await?;

                        let mut contents = vec![];
                        file.read_to_end(&mut contents).await?;

                        let mut gz = GzDecoder::new(&contents[..]);
                        let mut decoded = String::new();
                        gz.read_to_string(&mut decoded).unwrap();

                        let mut result = serde_json::from_str::<Vec<Document>>(&decoded)?;

                        if result.is_empty() {
                            warn!(logger, "Weird, found file with no content"; "file_name" => name.to_str().unwrap());
                        }

                        trace!(logger, "Found {} documents in file {}", result.len().to_string().yellow(), name.to_str().unwrap().yellow());

                        documents.append(&mut result);
                    }
                }
            }

            if documents.is_empty() {
                warn!(logger, "No documents found");
            }

            Ok(documents)
        }.boxed()
    }


    fn save_schema(&self, logger: &Logger, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string()));
        let base_path = self.base_path.clone();
        let schema = schema.clone();
        let all_schemas_fut = self.get_schemas(&logger);

        async move {
            let mut schemas = all_schemas_fut.await?;
            let schema_name = schema.name.to_owned();
            let schema_id = schema.id.to_owned();

            schemas.push(schema);

            if !Path::new(&base_path).is_dir() {
                create_dir(Path::new(&base_path)).await?;
            }

            let path = Path::new(&base_path).join("schemas.json");
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .await?;

            let data = serde_json::to_string_pretty(&schemas)?;

            file.write_all(&data.as_bytes()).await?;

            create_dir(Path::new(&base_path).join(&schema_name)).await?;

            debug!(logger, "Saved schema \"{}\"", schema_name; "id" => schema_id.to_string(), "name" => &schema_name);

            Ok(())
        }.boxed()
    }

    fn save_collection<'a>(&'a self, logger: &'a Logger, schema: &'a Schema, collection: &'a Collection) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> {
        let logger = logger.new(o!("schema_name" => schema.name.to_string(), "schema_id" => schema.id.to_string(), "collection_name" => collection.name.to_string(), "collection_id" => collection.id.to_string()));
        debug!(logger, "Saving collection for schema {}", schema.name);
        async move {
            let mut collections = self.collections.lock().await;

            match collections.get_mut(&schema.name) {
                Some(list) => {
                    list.push(collection.clone());
                },
                None => {
                    collections.insert(schema.name.to_string(), vec![collection.clone()]);
                }
            }
            Ok(())
        }.boxed()
    }

    fn save_document<'a>(&'a self, _logger: &'a Logger, schema: &'a Schema, collection: &'a Collection, document: Document) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> {
        async move {
            let mut documents = self.documents.lock().await;

            match documents.get_mut(&schema.name) {
                Some(schemas) => {
                    match schemas.get_mut(&collection.name) {
                        Some(collections) => {
                            collections.push(document);
                        },
                        None => {
                            schemas.insert(collection.name.to_string(), vec![document]);
                        }
                    }
                },
                None => {
                    let mut schemas = HashMap::new();
                    schemas.insert(collection.name.to_string(), vec![document]);
                    documents.insert(schema.name.to_string(), schemas);
                }
            }
            Ok(())
        }.boxed()
    }

    fn flush<'a>(&'a self, logger: &'a Logger) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> {
        async move {
            info!(logger, "Writing data to disk");

            // STEP 1 - Save Collections
            {
                let schemas = mem::replace(&mut *self.collections.lock().await, HashMap::new());
                for (schema_name, collections) in schemas {
                    info!(logger, "Saving collections for schema {}", schema_name);
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

                    debug!(logger, "Saved collections")
                }
            }

            // STEP 2 - Save documents
            {
                let documents = mem::replace(&mut *self.documents.lock().await, HashMap::new());
                for (schema_name, collections) in documents {
                    for (collection_name, documents) in collections {
                        info!(logger, "Saving documents for collection {} in schema {}", collection_name, &schema_name);
                        let base_path = Path::new(&self.base_path).join(&schema_name).join(format!("{}_docs", collection_name));
                        if !base_path.is_dir() {
                            create_dir(base_path.clone()).await?;
                        }

                        let chunks = documents.chunks(10_000);

                        let dir: Vec<_> = read_dir(base_path.clone()).await?.collect().await;

                        for (index, chunk) in chunks.into_iter().enumerate() {

                            match dir.iter().find(|i| i.as_ref().unwrap().file_name().to_str().unwrap().starts_with(&format!("{}_", index))) {
                                Some(dir) => {
                                    let file_name: String= dir.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                                    let hash = u64::from_str(file_name.split("_").collect::<Vec<_>>()[1].split(".").collect::<Vec<_>>()[0]).unwrap();

                                    let data = serde_json::to_string(&chunk).unwrap();

                                    let mut s = DefaultHasher::new();
                                    data.hash(&mut s);
                                    let sum = s.finish();

                                    if sum != hash {
                                        let path = base_path.join(&format!("{}_{}.gz", index, sum));
                                        let mut file = OpenOptions::new()
                                            .write(true)
                                            .create(true)
                                            .open(path)
                                            .await?;

                                        let mut e = GzEncoder::new(Vec::new(), Compression::default());
                                        e.write_all(data.as_bytes())?;

                                        file.write_all(&e.finish().unwrap()).await?;
                                    } else {
                                        debug!(logger, "Chunk {} has not changed for collection {} in schema {}", index, collection_name, &schema_name);
                                    }
                                },
                                None => {

                                    let data = serde_json::to_string(&chunk).unwrap();

                                    let mut s = DefaultHasher::new();
                                    data.hash(&mut s);
                                    let sum = s.finish();

                                    let path = base_path.join(&format!("{}_{}.gz", index, sum));
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .open(path)
                                        .await?;


                                    let mut e = GzEncoder::new(Vec::new(), Compression::default());
                                    e.write_all(data.as_bytes())?;

                                    file.write_all(&e.finish().unwrap()).await?;
                                }
                            }
                        }

                    }
                }
            }


            Ok(())
        }.boxed()
    }
}