#![feature(iter_array_chunks)]

use std::collections::HashSet;

const START_LOWER: u8 = b'a' as u8 - 1;
const START_UPPER: u8 = b'A' as u8 - 1;

struct Rucksack<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(input: &'a str) -> Self {
        let (left, right) = input.split_at(input.len() / 2);

        Self { left, right }
    }
}

impl Rucksack<'_> {
    fn common_items(&self) -> HashSet<u32> {
        let left_set: HashSet<_> = self.left.chars().map(char_to_int).collect();
        let right_set: HashSet<_> = self.right.chars().map(char_to_int).collect();

        left_set
            .intersection(&right_set)
            .map(|x| x.to_owned() as u32)
            .collect()
    }
}

fn char_to_int(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u8) - START_LOWER
    } else {
        (c as u8) - START_UPPER + 26
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day3.txt").expect("Wrong path!");

    let part1 = &contents
        .lines()
        .map(|line| Rucksack::from(line).common_items().iter().sum::<u32>())
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let part2 = contents
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|line| line.chars().map(char_to_int).collect::<HashSet<_>>())
                .fold(HashSet::new(), |acc, x| {
                    if acc.is_empty() {
                        return x;
                    }

                    acc.intersection(&x).map(|el| el.to_owned()).collect()
                })
                .into_iter()
                .map(|el| el as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
