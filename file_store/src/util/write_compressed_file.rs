use failure::Error;
use flate2::{
    write::GzEncoder,
    Compression,
};
use std::path::Path;
use std::io::Write;
use tokio::{
    fs::{
        OpenOptions,
    }
};
use tokio::io::AsyncWriteExt;

pub async fn write_compressed_file(data: &str, path: &Path) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .await?;

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_bytes())?;

    file.write_all(&e.finish().unwrap()).await?;

    Ok(())
}
