use crate::util::file_to_string;
use itertools::Itertools;
use std::collections::HashSet;

pub fn one() {
    let input = file_to_string("input/day3.txt");
    let mut values = ('a'..='z').collect::<Vec<char>>();
    values.extend(('A'..='Z'));
    let mut sum = vec![];
    for line in input.lines() {
        let first = &line[..line.len() / 2];
        let second = &line[line.len() / 2..];
        let character = first
            .chars()
            .find_or_first(|item| second.contains(*item))
            .unwrap();
        sum.push(
            values
                .iter()
                .position(|&element| element == character)
                .unwrap()
                + 1,
        );
    }
    println!("{:?}", sum.iter().sum::<usize>())
}

pub fn two() {
    let mut sum = vec![];
    let mut input = file_to_string("input/day3.txt");
    let mut sets = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>());

    let mut values = ('a'..='z').collect::<Vec<char>>();
    values.extend(('A'..='Z'));
    while let (Some(first), Some(second), Some(third)) = (sets.next(), sets.next(), sets.next()) {
        let item = *first
            .intersection(&second)
            .map(|letter| *letter)
            .collect::<HashSet<char>>()
            .intersection(&third)
            .next()
            .unwrap();
        sum.push(values.iter().position(|&element| element == item).unwrap() + 1);
    }

    println!("{:?}", sum.iter().sum::<usize>())
}
