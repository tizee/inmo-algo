use clap::ArgEnum;
use std::{fmt::Display, str::FromStr, string::ParseError};

#[derive(Debug, Clone, PartialEq, ArgEnum)]
// I do not recommend use languages without useful data structures to solve algorithm problems
pub enum Lang {
    Rust,
    Cpp,
    Python3,
    Typescript,
    Javascript,
    Unknown,
}

impl Lang {
    pub fn to_extension(&self) -> String {
        match &self {
            Lang::Cpp => "cpp".to_string(),
            Lang::Rust => "rs".to_string(),
            Lang::Python3 => "py".to_string(),
            Lang::Typescript => "ts".to_string(),
            Lang::Javascript => "js".to_string(),
            Lang::Unknown => "unknown".to_string(),
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "cpp" => Lang::Cpp,
            "rs" => Lang::Rust,
            "py" => Lang::Python3,
            "js" => Lang::Javascript,
            "ts" => Lang::Typescript,
            _ => Lang::Unknown,
        }
    }
}

impl FromStr for Lang {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for Lang {
    fn from(l: &str) -> Self {
        let language = &l.to_ascii_lowercase();
        match language.as_str() {
            "rust" => Lang::Rust,
            "cpp" => Lang::Cpp,
            "python3" => Lang::Python3,
            "javascript" => Lang::Javascript,
            "typescript" => Lang::Typescript,
            _ => Lang::Unknown,
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Lang::Rust => f.write_str("Rust"),
            Lang::Cpp => f.write_str("C++"),
            Lang::Python3 => f.write_str("Python3"),
            Lang::Javascript => f.write_str("Javascript"),
            Lang::Typescript => f.write_str("Typescript"),
            Lang::Unknown => f.write_str("unknown"),
        }
    }
}
