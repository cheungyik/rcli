use anyhow::Result;
use std::{
    fs::File,
    io::{self, Read},
};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 动态分发
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin()) // windows下Ctrl+Z表示EOF，Linux下按Ctrl+D表示EOF
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
