use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Storage;

impl Storage {
    pub fn load_data_from_file<P: AsRef<Path>, S: DeserializeOwned>(p: P) -> Result<S> {
        let path = p.as_ref();
        let file = File::open(path).context(format!("failed to open {}", path.display()))?;
        let file_reader = BufReader::new(file);
        let data = serde_json::from_reader(file_reader)
            .with_context(|| format!("failed to parse data in {}", path.display()))?;
        Ok(data)
    }

    pub fn persist<P: AsRef<Path>, S: Serialize>(p: P, d: &S) -> Result<()> {
        let path = p.as_ref();
        let json_content =
            serde_json::to_string_pretty(d).context("failed to serialize into json")?;
        // overwrite
        fs::write(path, json_content).context("failed to write into file")?;
        Ok(())
    }
}
