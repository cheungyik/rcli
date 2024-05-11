use anyhow::Result;
use serde_json::Value;

use crate::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);
    // headers 为可变引用，所以需要 clone 一份
    let headers = reader.headers()?.clone();
    // 通过 zip 方法将 headers迭代器 和 record迭代器 合并成一个元组迭代器然后转换为 Value 类型
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output, content)?;
    Ok(())
}
