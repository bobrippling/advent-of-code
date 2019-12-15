use std::fs;

//mod lib;
//use lib::Word;
use crate::lib::Word;

pub fn bytes(path: &str) -> Result<Vec<Word>, Box<dyn std::error::Error>> {
    let s = fs::read_to_string(path)?;
    let v = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<
            Result<
                _, //Vec<Word>,
                _ //Box<dyn std::error::Error>
            >
        >()?;
    Ok(v)
}
