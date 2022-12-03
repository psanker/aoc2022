// Rock, Paper, Scissors is a modulo-3 game
//
// Opponent set: {R, P, S}
//
// TIE SET: {R, P, S} --> {0, 1, 2} --> (x + 0) % 3
// WINNING SET: {P, S, R} --> {1, 2, 0} --> (x + 1) % 3
// LOSING SET: {S, R, P} --> {2, 0, 1} --> (x + 2) % 3
//
// Fun fact: `%` is the _remainder_ operation, **not** modulo! To do modulo, you need to use
// .rem_euclid()

use std::{char, fs::read_to_string};

#[derive(Debug)]
struct Round1(u32, u32);

enum Instruction {
    Tie,
    Win,
    Loss,
}

impl Round1 {
    fn score(&self) -> u32 {
        let mut score: u32 = 0;

        if self.0 == self.1 {
            score = (self.1 + 1) + 3;
        } else if (self.0 + 1) % 3 == self.1 {
            score = (self.1 + 1) + 6;
        } else if (self.0 + 2) % 3 == self.1 {
            score = self.1 + 1;
        }

        return score;
    }

    fn decode(char: &str) -> u32 {
        if char == "A" || char == "X" {
            return 0;
        } else if char == "B" || char == "Y" {
            return 1;
        } else {
            return 2;
        }
    }
}

impl From<String> for Round1 {
    fn from(input: String) -> Self {
        let split = input
            .split(" ")
            .take(2)
            .map(|el| Round1::decode(el))
            .collect::<Vec<u32>>();

        Self(split[0].to_owned(), split[1].to_owned())
    }
}

struct Round2(u32, Instruction);

impl Round2 {
    fn score(&self) -> u32 {
        let out = match &self.1 {
            Instruction::Loss => (self.0 + 2) % 3 + 1,
            Instruction::Tie => self.0 + 3 + 1,
            Instruction::Win => (self.0 + 1) % 3 + 6 + 1,
        };

        out
    }

    fn decode_partner(partner: &str) -> u32 {
        // Input set is {A, B, C}
        let chr = partner.chars().take(1).collect::<Vec<char>>()[0];

        match chr {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Somehow not {{A, B, C}}?"),
        }
    }

    fn decode_instruction(instruction: &str) -> Instruction {
        // Input set is {X, Y, Z}
        let chr = instruction.chars().take(1).collect::<Vec<char>>()[0];

        match chr {
            'X' => Instruction::Loss,
            'Y' => Instruction::Tie,
            'Z' => Instruction::Win,
            _ => panic!("Somehow not {{X, Y, Z}}?"),
        }
    }
}

impl From<String> for Round2 {
    fn from(input: String) -> Self {
        let split = input.split(" ").take(2).collect::<Vec<&str>>();

        Self(
            Self::decode_partner(split[0]),
            Self::decode_instruction(split[1]),
        )
    }
}

fn main() {
    let input = read_to_string("inputs/day2.txt").unwrap();

    let scores = &input
        .lines()
        .map(|line| Round1::from(line.to_owned()))
        .map(|round| round.score())
        .collect::<Vec<u32>>();

    println!("Day 2 part 1 score: {}", scores.iter().sum::<u32>());

    let correct_scores = &input
        .lines()
        .map(|line| Round2::from(line.to_owned()))
        .map(|round| round.score())
        .collect::<Vec<u32>>();

    println!("Day 2 part 2 score: {}", correct_scores.iter().sum::<u32>());
}
