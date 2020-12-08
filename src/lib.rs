use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

pub fn read_nums(filename: &str) -> Result<Vec<isize>> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let nums: std::result::Result<Vec<isize>, ParseIntError> = input
        .split_whitespace()
        .map(|x| x.parse::<isize>())
        .collect();

    Ok(nums?)
}

pub mod day1;
