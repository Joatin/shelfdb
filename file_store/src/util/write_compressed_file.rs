use failure::Error;
use flate2::{
    write::GzEncoder,
    Compression,
};
use std::{
    io::Write,
    path::Path,
};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
};

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
