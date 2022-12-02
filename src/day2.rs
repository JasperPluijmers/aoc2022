use crate::util::file_to_string;
use std::str::FromStr;

pub fn one() {
    let score = file_to_string("input/example2.txt")
        .lines()
        .map(|line| {
            calculate_score(
                line.chars().nth(2).unwrap().to_string().parse().unwrap(),
                line.chars().nth(0).unwrap().to_string().parse().unwrap(),
            )
        })
        .sum::<usize>();
    println!("{}", score)
}

pub fn two() {
    let score = file_to_string("input/day2.txt")
        .lines()
        .map(|line| {
            calculate_score_two(
                line.chars().nth(0).unwrap().to_string().parse().unwrap(),
                line.chars().nth(2).unwrap().to_string().parse().unwrap(),
            )
        })
        .sum::<usize>();
    println!("{}", score)
}

fn calculate_score_two(they: Hand, outcome: Outcome) -> usize {
    let me = strategy(&they, &outcome);
    outcome.score() + me.score()
}

fn strategy(they: &Hand, outcome: &Outcome) -> Hand {
    match outcome {
        Outcome::WON => match they {
            Hand::ROCK => Hand::PAPER,
            Hand::PAPER => Hand::SCISSORS,
            Hand::SCISSORS => Hand::ROCK,
        },
        Outcome::LOSS => match they {
            Hand::ROCK => Hand::SCISSORS,
            Hand::PAPER => Hand::ROCK,
            Hand::SCISSORS => Hand::PAPER,
        },
        Outcome::DRAW => they.clone(),
    }
}
fn calculate_score(me: Hand, other: Hand) -> usize {
    me.beats(&other).score() + me.score()
}

#[derive(Clone, PartialEq, Debug)]
enum Hand {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Hand {
    fn beats(&self, other: &Hand) -> Outcome {
        if self == other {
            return Outcome::DRAW;
        }
        match self {
            Hand::ROCK if other == &Hand::SCISSORS => Outcome::WON,
            Hand::PAPER if other == &Hand::ROCK => Outcome::WON,
            Hand::SCISSORS if other == &Hand::PAPER => Outcome::WON,
            _ => Outcome::LOSS,
        }
    }

    fn score(&self) -> usize {
        match self {
            Hand::ROCK => 1,
            Hand::PAPER => 2,
            Hand::SCISSORS => 3,
        }
    }
}

enum Outcome {
    WON,
    LOSS,
    DRAW,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::WON => 6,
            Outcome::LOSS => 0,
            Outcome::DRAW => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::LOSS),
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::WON),
            _ => Err(()),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::ROCK),
            "B" | "Y" => Ok(Hand::PAPER),
            "C" | "Z" => Ok(Hand::SCISSORS),
            &_ => Err(()),
        }
    }
}
