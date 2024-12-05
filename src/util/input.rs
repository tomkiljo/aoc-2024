use std::{
    fs::File,
    io::{BufRead, Read},
};

use anyhow::Context;

pub fn read_file(path: &str) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path).with_context(|| format!("File {path} not found"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| "Failde to read file")?;
    Ok(buffer)
}

pub fn read_lines(buffer: &Vec<u8>) -> Vec<String> {
    let reader = std::io::BufReader::new(buffer.as_slice());
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn read_buf_ints(buffer: &Vec<u8>) -> Vec<i64> {
    buffer
        .split(|&c| !(c as char).is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8(s.to_vec()).unwrap().parse().unwrap())
        .collect()
}

pub fn read_str_ints(line: &str) -> Vec<i64> {
    line.split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
