use std::collections::HashSet;

use anyhow::Result;

fn get_start(line: &str, size: usize) -> usize {
    line.as_bytes()
        .windows(size)
        .position(|set| {
            let mut data = 0;

            for &c in set {
                let prev: usize = data;
                // Bit masking to see if character already exists
                data |= 1 << (c - b'a');

                if prev == data {
                    return false;
                }
            }

            true
        })
        .map(|i| i + size)
        .unwrap()
}

fn get_stream_start(line: &str) -> usize {
    get_start(line, 4)
}

fn get_message_start(line: &str) -> usize {
    get_start(line, 14)
}

fn main() -> Result<()> {
    let streams = std::fs::read_to_string("inputs/day6.prod")?;

    let stream_starts: Vec<_> = streams.lines().map(get_stream_start).collect();
    println!("Start part 1: {:?}", stream_starts);

    let message_starts: Vec<_> = streams.lines().map(get_message_start).collect();
    println!("Start part 2: {:?}", message_starts);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_message_start, get_stream_start};

    use anyhow::Result;

    #[test]
    fn test_one() -> Result<()> {
        let streams = std::fs::read_to_string("inputs/day6.test")?;

        let starts: Vec<_> = streams.lines().map(get_stream_start).collect();

        assert_eq!(starts, vec![7, 5, 6, 10, 11]);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let streams = std::fs::read_to_string("inputs/day6.test")?;

        let starts: Vec<_> = streams.lines().map(get_message_start).collect();

        assert_eq!(starts, vec![19, 23, 23, 29, 26]);
        Ok(())
    }
}
