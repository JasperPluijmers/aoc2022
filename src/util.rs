use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn file_to_vector<A: FromStr + Debug>(filename: &str) -> Vec<A>
where
    <A as FromStr>::Err: Debug,
{
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|row| row.unwrap().parse::<A>().unwrap())
        .collect::<Vec<A>>()
}

pub fn file_to_string(filename: &str) -> String {
    let mut input = String::new();
    BufReader::new(File::open(filename).unwrap())
        .read_to_string(&mut input)
        .unwrap();
    input
}
