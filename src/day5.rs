use crate::util::file_to_string;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Range, RangeInclusive};

fn execute_line(
    stacks: &mut HashMap<usize, VecDeque<char>>,
    amount: usize,
    from: usize,
    to: usize,
) {
    for _ in 0..amount {
        let letter = stacks.get_mut(&from).unwrap().pop_front().unwrap();
        stacks.get_mut(&to).unwrap().push_front(letter);
    }
}

fn calculate_final(mut stacks: HashMap<usize, VecDeque<char>>) -> Vec<String> {
    let max = stacks.keys().max().unwrap();
    (1..max + 1)
        .map(|index| {
            stacks
                .get_mut(&index)
                .unwrap()
                .pop_front()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}

fn create_initial(initial: &str) -> HashMap<usize, VecDeque<char>> {
    let mut stacks = HashMap::new();
    for line in initial.lines() {
        line.chars()
            .skip(1)
            .enumerate()
            .filter(|(index, letter)| ('A'..='Z').contains(letter))
            .for_each(|(index, letter)| {
                stacks
                    .entry((index / 4) + 1)
                    .or_insert(VecDeque::new())
                    .push_back(letter)
            });
    }
    stacks
}
pub fn one() {
    let input = file_to_string("input/day5.txt");
    let mut parts = input.split("\n\n");
    let initial = parts.next().unwrap();
    let instructions = parts.next().unwrap();
    let mut stacks = create_initial(initial);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in instructions.lines() {
        let captures = re.captures(line).unwrap();
        execute_line(
            &mut stacks,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );
    }
    println!("{:?}", calculate_final(stacks).join(""));
}

pub fn two() {
    let input = file_to_string("input/day5.txt");
    let mut parts = input.split("\n\n");
    let initial = parts.next().unwrap();
    let instructions = parts.next().unwrap();
    let mut stacks = create_initial(initial);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in instructions.lines() {
        let captures = re.captures(line).unwrap();
        execute_line_second(
            &mut stacks,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );
    }
    println!("{:?}", calculate_final(stacks).join(""))
}

fn execute_line_second(
    stacks: &mut HashMap<usize, VecDeque<char>>,
    amount: usize,
    from: usize,
    to: usize,
) {
    let letters = stacks
        .get_mut(&from)
        .unwrap()
        .drain(0..amount)
        .collect::<VecDeque<char>>();
    let to_stack = stacks.get_mut(&to).unwrap();
    letters
        .iter()
        .rev()
        .for_each(|item| to_stack.push_front(*item))
}
