mod b64;
mod csv;
mod genpass;

use anyhow::Result;
use clap::Parser;
use std::path::Path;

pub use self::{
    b64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
use self::{csv::CsvOpts, genpass::GenPassOpts};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(
        name = "csv",
        about = "Show CSV, or Convert CSV to ohre formats(Json, Yaml)"
    )]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into()) // into() 将 &str 转换为 String
    } else {
        anyhow::bail!("File not found: {}", filename)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-").unwrap(), "-");
        assert_eq!(
            verify_input_file("*").unwrap_err().to_string(),
            "File not found: *"
        );
        assert_eq!(verify_input_file("Cargo.toml").unwrap(), "Cargo.toml");
        assert_eq!(
            verify_input_file("not-exist").unwrap_err().to_string(),
            "File not found: not-exist"
        );
        assert_eq!(
            verify_input_file("hi.toml").unwrap_err().to_string(),
            "File not found: hi.toml"
        );
    }
}
