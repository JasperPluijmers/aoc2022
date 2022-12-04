use crate::util::file_to_string;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

fn parse_line(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let mut parts = line.split(',');
    let (mut first, mut second) = (
        parts.next().unwrap().split('-'),
        parts.next().unwrap().split('-'),
    );
    (
        first.next().unwrap().parse::<usize>().unwrap()
            ..=first.next().unwrap().parse::<usize>().unwrap(),
        second.next().unwrap().parse::<usize>().unwrap()
            ..=second.next().unwrap().parse::<usize>().unwrap(),
    )
}
pub fn one() {
    let input = file_to_string("input/day4.txt");
    let sum = input
        .lines()
        .map(|line| parse_line(line))
        .filter(|(first, second)| {
            first.clone().all(|number| second.contains(&number))
                || second.clone().all(|number| first.contains(&number))
        })
        .count();

    println!("{:?}", sum)
}

pub fn two() {
    let input = file_to_string("input/day4.txt");
    let sum = input
        .lines()
        .map(|line| parse_line(line))
        .filter(|(first, second)| first.clone().any(|number| second.contains(&number)))
        .count();

    println!("{:?}", sum)
}
