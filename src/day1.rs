use crate::util::file_to_string;
use itertools::Itertools;

pub fn one() {
    let max = file_to_string("input/day1.txt")
        .trim()
        .split("\n\n")
        .map(|list| {
            list.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max();
    println!("{:?}", max)
}

pub fn two() {
    let calories = file_to_string("input/day1.txt")
        .trim()
        .split("\n\n")
        .map(|list| {
            list.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum::<usize>();
    println!("{:?}", calories);
}

fn get_calories() -> impl Iterator<Item = usize> {
    let file = file_to_string("input/day1.txt");
    let calories = file
        .trim()
        .split("\n\n")
        .map(|list| {
            list.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>()
        .into_iter();
    calories
}
