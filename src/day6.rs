use crate::util::file_to_string;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

pub fn one() {
    let input = file_to_string("input/day6.txt")
        .chars()
        .collect::<Vec<char>>();
    println!("{:?}", find_start_of_message(input, 4))
}

pub fn two() {
    let input = file_to_string("input/day6.txt")
        .chars()
        .collect::<Vec<char>>();
    println!("{:?}", find_start_of_message(input, 14))
}

fn find_start_of_message(input: Vec<char>, size: usize) -> usize {
    input
        .windows(size)
        .enumerate()
        .find(|(index, window)| window.iter().collect::<HashSet<&char>>().len() == size)
        .unwrap()
        .0
        + size
}
