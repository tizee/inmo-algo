use anyhow::{anyhow, Context, Result};
use serde_derive::{Deserialize, Serialize};

use crate::common::Lang;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// leetcode root directory
    /// default to $HOME/inmo/leetcode
    pub leetcode: PathBuf,
    /// codeforces root directory
    /// default to $HOME/inmo/codeforces
    pub codeforces: PathBuf,
    /// cache root dir
    /// default to $HOME/.inmo
    pub cache: PathBuf,
    #[serde(default)]
    /// default to cpp
    pub default_lang: Lang,
}

pub struct ConfigPaths;

impl ConfigPaths {
    pub fn config_dir() -> PathBuf {
        dirs_next::home_dir().unwrap().join(".config").join("inmo")
    }
    pub fn default_cache_dir() -> PathBuf {
        dirs_next::home_dir().unwrap().join(".inmo")
    }
    pub fn config_file_path() -> PathBuf {
        ConfigPaths::config_dir().join("config.toml")
    }

    pub fn default_data_path() -> PathBuf {
        dirs_next::home_dir().unwrap().join("inmo")
    }
}

fn read_config<R: Read>(config: R) -> Result<Config> {
    let mut reader = BufReader::new(config);
    let mut buf = Vec::new();
    reader
        .read_to_end(&mut buf)
        .context("failed to read config")?;
    match toml::from_slice::<Config>(&buf) {
        Ok(config) => Ok(config),
        Err(err) => Err(anyhow!("{}", err)),
    }
}
pub fn load_default_config() -> Result<Config> {
    generate_default_config().context("failed to generate default config")?;
    load_config(ConfigPaths::config_file_path())
}

fn load_config<P: AsRef<Path>>(p: P) -> Result<Config> {
    let path = p.as_ref();
    if path.exists() && path.is_file() {
        return match File::open(path) {
            Ok(file) => read_config(file),
            Err(err) => Err(anyhow!("{}", err)),
        };
    }
    Err(anyhow!(format!(
        "{}: doesn't exists or is not a file",
        path.display()
    )))
}

/// create config
pub fn generate_config() -> Result<()> {
    let default_config_path = ConfigPaths::config_file_path();
    let config_dir = ConfigPaths::config_dir();
    if default_config_path.exists() {
        println!(
            "inmo config file exists at: {}",
            default_config_path.display()
        );
        print!("Overwrite? (y/N): ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => eprintln!("{:?}", err),
        };
        // ask for overwrite?
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(err) => eprintln!("{:?}", err),
        };
        if !buf.trim().eq_ignore_ascii_case("Y") {
            return Ok(());
        }
    } else {
        fs::create_dir_all(config_dir).context("failed to create dir for config file")?;
    }
    let default_config =
        toml::to_string(&Config::default()).context("failed to generate default config")?;
    fs::write(default_config_path, default_config).context("failed to create config file")?;
    Ok(())
}

/// create config if not found
pub fn generate_default_config() -> Result<()> {
    let default_config_path = ConfigPaths::config_file_path();
    if !default_config_path.exists() {
        println!("generate default config");
        return generate_config();
    }
    Ok(())
}

impl Default for Config {
    fn default() -> Self {
        Config {
            leetcode: ConfigPaths::default_data_path().join("leetcode"),
            codeforces: ConfigPaths::default_data_path().join("codeforces"),
            cache: ConfigPaths::default_cache_dir(),
            default_lang: Lang::Cpp,
        }
    }
}

#[cfg(test)]
mod test_config {
    use super::{read_config, Config};
    #[test]
    fn test_cfg_io() {
        let default_cfg = Config::default();
        let cfg_str = toml::to_string(&default_cfg).unwrap();
        let cfg = read_config(cfg_str.as_bytes()).unwrap();
        let cfg_str2 = toml::to_string(&cfg).unwrap();
        println!("{}", cfg_str2);
        assert_eq!(cfg_str, cfg_str2);
    }
}
