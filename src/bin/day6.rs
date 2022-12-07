use std::collections::VecDeque;

use anyhow::Result;
use window::Window;

mod window {
    use std::{
        collections::{HashSet, VecDeque},
        hash::Hash,
    };

    pub struct Window<T: Eq + Hash> {
        data: VecDeque<T>,
        size: usize,
    }

    impl<T: Eq + Hash> Window<T> {
        pub fn new(size: usize) -> Self {
            Self {
                data: VecDeque::new(),
                size,
            }
        }

        pub fn push(&mut self, item: T) -> Option<T> {
            let out = if self.is_full() {
                self.data.pop_back()
            } else {
                None
            };

            self.data.push_front(item);
            out
        }

        pub fn contains(&self, item: &T) -> bool {
            self.data.contains(&item)
        }

        pub fn is_full(&self) -> bool {
            self.data.len() == self.size
        }

        pub fn is_uniquely_full(&self) -> bool {
            self.data.iter().collect::<HashSet<_>>().len() == self.size
        }
    }
}

fn get_start(line: &str, size: usize) -> u32 {
    let mut buffer = Window::new(size);
    let mut idx = 0;

    let mut line_chars = line.chars();

    while let Some(c) = line_chars.next() {
        if buffer.is_uniquely_full() {
            break;
        }

        buffer.push(c);
        idx += 1;
    }

    idx

}

fn get_stream_start(line: &str) -> u32 {
    get_start(line, 4)
}

fn get_message_start(line: &str) -> u32 {
    get_start(line, 14)
}

fn main() -> Result<()> {
    let streams = std::fs::read_to_string("inputs/day6.prod")?;

    let stream_starts: Vec<_> = streams
        .lines()
        .map(get_stream_start)
        .collect();

    println!("Start part 1: {:?}", stream_starts);

    let message_starts: Vec<_> = streams
        .lines()
        .map(get_message_start)
        .collect();

    println!("Start part 2: {:?}", message_starts);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_stream_start, get_message_start};

    use anyhow::Result;

    #[test]
    fn test_one() -> Result<()> {
        let streams = std::fs::read_to_string("inputs/day6.test")?;

        let starts: Vec<_> = streams
            .lines()
            .map(get_stream_start)
            .collect();

        assert_eq!(starts, vec![7, 5, 6, 10, 11]);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let streams = std::fs::read_to_string("inputs/day6.test")?;

        let starts: Vec<_> = streams
            .lines()
            .map(get_message_start)
            .collect();

        assert_eq!(starts, vec![19, 23, 23, 29, 26]);
        Ok(())
    }
}
