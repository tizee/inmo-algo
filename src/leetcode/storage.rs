use super::problem::LCProblem;
use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Storage;

impl Storage {
    pub fn load_from_file<P: AsRef<Path>>(p: P) -> Result<Vec<LCProblem>> {
        let path = p.as_ref();
        let file = File::open(path).context(format!("failed to open {}", path.display()))?;
        let file_reader = BufReader::new(file);
        let list = serde_json::from_reader(file_reader)
            .with_context(|| format!("failed to parse data in {}", path.display()))?;
        Ok(list)
    }

    pub fn persist<P: AsRef<Path>>(p: P, list: &Vec<LCProblem>) -> Result<()> {
        let path = p.as_ref();
        let json_content =
            serde_json::to_string_pretty(list).context("failed to serialize list into json")?;
        // overwrite
        fs::write(path, json_content).context("failed to write into file")?;
        Ok(())
    }
}
