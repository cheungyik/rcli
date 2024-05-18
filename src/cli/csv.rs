use anyhow::{Error, Result};
use clap::Parser;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::CmdExecutor;

use super::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl CmdExecutor for CsvOpts {
    async fn execute(self) -> Result<()> {
        let output = self.output.unwrap_or(format!("output.{}", self.format));
        crate::process_csv(&self.input, &output, self.format)
    }
}

fn parse_output_format(format: &str) -> Result<OutputFormat, Error> {
    format.parse::<OutputFormat>()
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

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", format)),
        }
    }
}

// 实现 Display trait 以使用 {} 占位符输出 OutputFormat 类型
impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self)) // 解引用并转换为 &str 类型
    }
}
