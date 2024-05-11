use anyhow::{Ok, Result};
use std::fs::File;

pub fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    // 动态分发
    let reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin()) // windows下Ctrl+Z表示EOF，Linux下按Ctrl+D表示EOF
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
