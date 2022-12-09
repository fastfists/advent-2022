use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Self::Rock),
            "Y" | "B" => Ok(Self::Paper),
            "Z" | "C" => Ok(Self::Scissors),
            _ => panic!("Impossible")
        }
    }
}

fn score_game(other: &Choice, me: &Choice) -> i32 {

    let extra_points = match me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    match (me, other) {
        (Choice::Rock, Choice::Scissors) => extra_points+6,
        (Choice::Scissors, Choice::Paper) => extra_points+6,
        (Choice::Paper, Choice::Rock) => extra_points+6,

        (Choice::Scissors, Choice::Rock) => extra_points,
        (Choice::Paper, Choice::Scissors) => extra_points,
        (Choice::Rock, Choice::Paper) => extra_points,

        (Choice::Scissors, Choice::Scissors) => extra_points+3,
        (Choice::Rock, Choice::Rock) => extra_points+3,
        (Choice::Paper, Choice::Paper) => extra_points+3,
    }
}

fn score_end(other: &Choice, end: &Choice) -> i32 {

    let me = match (end, other) {
        (Choice::Rock, Choice::Scissors) => Choice::Paper,        // LOSE
        (Choice::Rock, Choice::Rock) => Choice::Scissors,
        (Choice::Rock, Choice::Paper) => Choice::Rock,

        (Choice::Paper, Choice::Scissors) => Choice::Scissors,       // TIE
        (Choice::Paper, Choice::Rock) => Choice::Rock,
        (Choice::Paper, Choice::Paper) => Choice::Paper,

        (Choice::Scissors, Choice::Scissors) => Choice::Rock,
        (Choice::Scissors, Choice::Rock) => Choice::Paper,
        (Choice::Scissors, Choice::Paper) => Choice::Scissors,    // WIN
    };

    score_game(other, &me)
}

fn main() {
    let output1 :i32  = include_str!("input")
        .lines()
        .map( |line| line.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<Choice>>()   )
        .map(|game| score_game(game.get(0).unwrap(), game.get(1).unwrap()))
        .sum();

    let output2 :i32  = include_str!("input")
        .lines()
        .map( |line| line.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<Choice>>()   )
        .map(|game| score_end(game.get(0).unwrap(), game.get(1).unwrap()))
        .sum();

    println!("{output1:?} ");
    println!("{output2:?} ");
}
