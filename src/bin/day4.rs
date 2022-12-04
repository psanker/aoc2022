use std::collections::HashSet;

fn main() {
    let contents = std::fs::read_to_string("inputs/day4.txt")
        .expect("Missing file!")
        .lines()
        .map(|line| {
            let ranges = line
                .split(",")
                .take(2)
                .map(|range| {
                    let endpoints = range
                        .split("-")
                        .take(2)
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

                    (endpoints[0]..=endpoints[1]).collect::<HashSet<_>>()
                })
                .collect::<Vec<HashSet<_>>>();
            ranges
        })
        .collect::<Vec<_>>();

    let part1: u32 = contents
        .iter()
        .map(|ranges| {
            ((ranges[0].is_subset(&ranges[1]) || ranges[1].is_subset(&ranges[0])) as u8) as u32
        })
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let part2: u32 = contents
        .iter()
        .map(|ranges| ((ranges[0].intersection(&ranges[1]).count() > 0) as u8) as u32)
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
