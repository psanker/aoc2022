use std::{fs::read_to_string, io::Error};

fn day1pt1(path: &str) -> Result<i32, Error> {
    let totals = get_totals(path)?;

    let max = totals.into_iter().fold(0, |acc, x| acc.max(x));
    Ok(max)
}

fn day1pt2(path: &str) -> Result<i32, Error> {
    let mut totals = get_totals(path)?;
    totals.sort();
    let n = totals.len();

    let top3_sum = totals[(n-3)..n].into_iter().fold(0, |acc, x| acc + x);
    Ok(top3_sum)
}

fn get_totals(path: &str) -> Result<Vec<i32>, Error> {
    let contents = read_to_string(path)?;
    let mut buffer = vec![];
    let mut totals = vec![];

    contents.split("\n").for_each(|el| {
        if el == "" {
            let sum = buffer.clone().into_iter().fold(0, |acc, x| acc + x);
            totals.push(sum);
            buffer.clear();
        } else {
            let val: i32 = el.parse().expect("This should be an integer!");
            buffer.push(val);
        }
    });

    Ok(totals)
}

fn main() {
    let d1p1 = day1pt1("inputs/day1.txt").ok().unwrap();
    println!("Day 1 pt 1: {}", d1p1);

    let d1p2 = day1pt2("inputs/day1.txt").ok().unwrap();
    println!("Day 1 pt 2: {}", d1p2);

    let (d1p1_better, d1p2_better) = day1_better("inputs/day1.txt").ok().unwrap();
    println!("Day 1 better: ({}, {})", d1p1_better, d1p2_better);
}

fn day1_better(path: &str) -> Result<(i32, i32), Error>{
    let mut totals = read_to_string(path)?
        .split("\n\n")
        .map(|chunk| chunk.lines().flat_map(|line| line.parse::<i32>()).sum())
        .collect::<Vec<i32>>();

    totals.sort();
    totals.reverse();

    Ok((totals[0], totals.iter().take(3).sum()))
}
