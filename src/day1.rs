use std::{fs::read_to_string, io::Error};

pub fn day1pt1(path: &str) -> Result<i32, Error> {
    let totals = get_totals(path)?;

    let max = totals.into_iter().fold(0, |acc, x| acc.max(x));
    Ok(max)
}

pub fn day1pt2(path: &str) -> Result<i32, Error> {
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
