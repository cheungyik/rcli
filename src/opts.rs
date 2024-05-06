use anyhow::{Error, Ok, Result};
use clap::Parser;
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

// 数据量小且在栈上分配的类型可以实现 Copy trait
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 实现了From trait之后，可以使用Into trait将OutputFormat类型转换为&str类型
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 为 OutputFormat 类型实现 FromStr trait 以便将字符串转换为 OutputFormat 类型
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Invalid output format: {}", format),
        }
    }
}

// 实现 Display trait 以使用 {} 占位符输出 OutputFormat 类型
impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self)) // 解引用并转换为 &str 类型
    }
}

fn verify_input_file(filename: &str) -> Result<String> {
    if Path::new(filename).exists() {
        Ok(filename.into()) // into() 将 &str 转换为 String
    } else {
        anyhow::bail!("File not found: {}", filename)
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, Error> {
    format.parse::<OutputFormat>()
}
