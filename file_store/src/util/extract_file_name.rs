use failure::Error;
use std::{
    io,
    str::FromStr,
};
use tokio::fs::DirEntry;

pub fn extract_file_name(dir: &Result<DirEntry, io::Error>) -> Result<(String, u64), Error> {
    if let Ok(ref_dir) = dir.as_ref() {
        if let Some(raw_text) = ref_dir.file_name().to_str() {
            let full_name = raw_text.to_string();
            let hash = u64::from_str(
                full_name.split('_').collect::<Vec<_>>()[1]
                    .split('.')
                    .collect::<Vec<_>>()[0],
            )
            .unwrap();

            return Ok((full_name, hash));
        }
    }

    bail!("Could not extract file name")
}
