#![allow(unused, unused_imports, dead_code)]

use std::env;
use aoc_2022::io as our_io;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filename = env::args()
        .nth(1)
        .expect("No filename given");

    let input = our_io::read_lines(filename)?
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let score = input
        .iter()
        .map(|line| line.split_whitespace())
        // transform each line into a tuple of hands so we can actually do something with it
        .map(|mut hands| {
            let first = hands.next().unwrap();
            let second = hands.next().unwrap();

            (opponent_str_to_hand(first), your_str_to_hand(second))
        })
        .fold(0, |mut accum, (opponent, yours)| {
            accum += play_round(&opponent, &yours);
            accum
        });

    let part2_score = input
        .iter()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            let first = line.next().unwrap();
            let second = line.next().unwrap();

            let opponent_hand = opponent_str_to_hand(&first);
            let desired_outcome = str_to_outcome(&second);

            (opponent_hand, desired_hand_by_goal(&opponent_hand, &desired_outcome))
        })
        .fold(0, |mut accum, (opponent, yours)| {
            accum += play_round(&opponent, &yours);
            accum
        });

    println!("score: {}", score);
    println!("part2 score: {}", part2_score);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock = 1,
    Paper  = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Goal {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn desired_hand_by_goal(opponent: &Hand, goal: &Goal) -> Hand {
    match (opponent, goal) {
        (Hand::Rock, Goal::Win) => Hand::Paper,
        (Hand::Rock, Goal::Lose) => Hand::Scissors,
        (Hand::Rock, Goal::Draw) => Hand::Rock,

        (Hand::Paper, Goal::Win) => Hand::Scissors,
        (Hand::Paper, Goal::Lose) => Hand::Rock,
        (Hand::Paper, Goal::Draw) => Hand::Paper,

        (Hand::Scissors, Goal::Win) => Hand::Rock,
        (Hand::Scissors, Goal::Lose) => Hand::Paper,
        (Hand::Scissors, Goal::Draw) => Hand::Scissors,
    }
}

fn str_to_outcome(s: &str) -> Goal {
    match s {
        "X" => Goal::Lose,
        "Y" => Goal::Draw,
        "Z" => Goal::Win,
        _ => panic!("Unknown outcome: {}", s),
    }
}

fn opponent_str_to_hand(s: &str) -> Hand {
    match s {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Invalid hands"),
    }
}

fn your_str_to_hand(s: &str) -> Hand {
    match s {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("Invalid hands"),
    }
}

fn play_round(opponent: &Hand, yours: &Hand) -> u32 {
    match (opponent, yours) {
        (Hand::Rock, Hand::Rock) => 1 + 3 ,
        (Hand::Rock, Hand::Paper) => 2 + 6 ,
        (Hand::Rock, Hand::Scissors) => 3 + 0,

        (Hand::Paper, Hand::Rock) => 1 + 0,
        (Hand::Paper, Hand::Paper) => 2 + 3,
        (Hand::Paper, Hand::Scissors) => 3 + 6,

        (Hand::Scissors, Hand::Rock) => 1 + 6,
        (Hand::Scissors, Hand::Paper) => 2 + 0,
        (Hand::Scissors, Hand::Scissors) => 3 + 3,
        _ => unreachable!(),
    }
}
