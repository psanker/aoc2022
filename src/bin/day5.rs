use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    usize,
};

use anyhow::{anyhow, Result};
use regex::Regex;

type Stack<T> = VecDeque<T>;

#[derive(Debug, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Crane {
    boxes: HashMap<usize, Stack<char>>,
    instructions: VecDeque<Instruction>,
    original_state: Option<Box<Crane>>,
}

impl Crane {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
            instructions: VecDeque::new(),
            original_state: None,
        }
    }

    fn save(&mut self) -> Result<()> {
        if self.original_state.is_some() {
            return Err(anyhow!(
                "Need to .reset() the crane before simulating again!"
            ));
        }

        let original = self.clone();
        self.original_state = Some(Box::new(original));

        Ok(())
    }

    fn simulate_one(&mut self) -> Result<()> {
        self.save()?;

        while let Some(instruction) = self.instructions.pop_front() {
            let mut count = instruction.amount;

            while count > 0 {
                count = self.move_one(&instruction, count);
            }
        }

        Ok(())
    }

    fn simulate_two(&mut self) -> Result<()> {
        self.save()?;

        while let Some(instruction) = self.instructions.pop_front() {
            println!("Step -----------");
            println!("{:?}", &instruction);
            println!(
                "Old {}: {}",
                &instruction.from,
                self.boxes
                    .get(&instruction.from)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            );
            println!(
                "Old {}: {}",
                &instruction.to,
                self.boxes
                    .get(&instruction.to)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            );
            let mut count = instruction.amount.clone();

            while count > 0 {
                count = self.move_three(&instruction, count);
            }
            println!(
                "New {}: {}",
                &instruction.from,
                self.boxes
                    .get(&instruction.from)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            );
            println!(
                "New {}: {}",
                &instruction.to,
                self.boxes
                    .get(&instruction.to)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            );
        }

        Ok(())
    }

    fn move_one(&mut self, instruction: &Instruction, count: usize) -> usize {
        let current_box = self
            .boxes
            .get_mut(&instruction.from)
            .expect("Missing column?")
            .pop_front()
            .expect("Popped from an empty stack :(");

        self.boxes
            .entry(instruction.to.clone())
            .and_modify(|v| v.push_front(current_box));

        count - 1
    }

    fn move_three(&mut self, instruction: &Instruction, count: usize) -> usize {
        let subcount = count;

        let current_col = self
            .boxes
            .get_mut(&instruction.from)
            .expect("Missing column?");

        let mut tmp: VecDeque<_> = current_col.drain(..subcount).collect();
        println!("Drained: {:?}", tmp.iter().collect::<String>());

        let target_col = self
            .boxes
            .get_mut(&instruction.to)
            .expect("Missing target column?");

        tmp.append(target_col);

        _ = self.boxes.entry(instruction.to).and_modify(|v| *v = tmp);

        count - subcount
    }

    fn top_crates(&self) -> String {
        let mut chars = Vec::new();
        chars.resize(self.boxes.len(), ' ');

        let tops: Vec<_> = self
            .boxes
            .iter()
            .map(|(k, v)| (k.clone(), v.front().map_or(' ', |c| c.clone())))
            .collect();

        for (k, v) in tops.into_iter() {
            chars[k - 1] = v;
        }

        chars.into_iter().collect()
    }

    fn reset(&mut self) {
        if let Some(box_orig) = self.original_state.take() {
            let original: Crane = *box_orig;

            self.boxes = original.boxes;
            self.instructions = original.instructions;
            self.original_state = None;
        }
    }

    fn parse_line(&mut self, line: &str) {
        if line.starts_with(" 1") || line.is_empty() {
            return;
        }

        if line.starts_with("move") {
            self.parse_move(line);
        } else {
            self.parse_box_row(line);
        }
    }

    fn parse_box_row(&mut self, line: &str) {
        let mut space_counter = 0;
        let mut col_idx = 0;

        let mut chars_iter = line.chars();

        while let Some(c) = chars_iter.next() {
            if c == ' ' {
                space_counter += 1;

                if space_counter == 4 {
                    col_idx += 1;
                    space_counter = 0;
                }
            } else if c == '[' {
                // Reset space counter in case there was a situation like
                // [a]_[b]_[c]
                space_counter = 0;

                // There is always a char after [
                // and a ] after the char
                let letter = chars_iter.next().unwrap();
                _ = self
                    .boxes
                    .entry(col_idx + 1)
                    .and_modify(|v| v.push_back(letter))
                    .or_insert(VecDeque::from([letter]));

                col_idx += 1;

                // Advance to the ]
                _ = chars_iter.next().unwrap();
            }
        }
    }

    fn parse_move(&mut self, line: &str) {
        let pat = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        for cap in pat.captures_iter(line) {
            let amount = cap[1].parse().unwrap();
            let from = cap[2].parse().unwrap();
            let to = cap[3].parse().unwrap();

            self.instructions
                .push_back(Instruction { amount, from, to });
        }
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crane = Crane::new();

        for line in s.lines() {
            crane.parse_line(line);
        }

        Ok(crane)
    }
}

fn main() -> Result<()> {
    let mut crane: Crane = std::fs::read_to_string("inputs/day5.prod")?.parse()?;

    println!("{:?}", crane.boxes);

    crane.simulate_one()?;

    println!("Top crates: '{}'", crane.top_crates());

    crane.reset();
    crane.simulate_two()?;
    println!("Top crates: '{}'", crane.top_crates());

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    fn view_stack(crane: &super::Crane, i: &usize) -> String {
        format!("{:?}", crane.boxes.get(i).unwrap())
    }

    #[test]
    fn test_one() -> Result<()> {
        let mut crane: super::Crane = std::fs::read_to_string("inputs/day5.test")?.parse()?;

        assert_eq!(view_stack(&crane, &1), "['N', 'Z']");
        assert_eq!(view_stack(&crane, &2), "['D', 'C', 'M']");
        assert_eq!(view_stack(&crane, &3), "['P']");

        crane.simulate_one()?;

        assert_eq!(view_stack(&crane, &1), "['C']");
        assert_eq!(view_stack(&crane, &2), "['M']");
        assert_eq!(view_stack(&crane, &3), "['Z', 'N', 'D', 'P']");

        Ok(())
    }

    #[test]
    fn test_prod_one() -> Result<()> {
        let mut crane: super::Crane = std::fs::read_to_string("inputs/day5.prod")?.parse()?;

        assert_eq!(view_stack(&crane, &1), "['R', 'H', 'M', 'P', 'Z']");
        assert_eq!(view_stack(&crane, &2), "['B', 'J', 'C', 'P']");
        assert_eq!(
            view_stack(&crane, &3),
            "['D', 'C', 'L', 'G', 'H', 'N', 'S']"
        );
        assert_eq!(
            view_stack(&crane, &4),
            "['L', 'R', 'S', 'Q', 'D', 'M', 'T', 'F']"
        );
        assert_eq!(
            view_stack(&crane, &5),
            "['M', 'Z', 'T', 'B', 'Q', 'P', 'S', 'F']"
        );
        assert_eq!(view_stack(&crane, &6), "['G', 'B', 'Z', 'S', 'F', 'T']");
        assert_eq!(view_stack(&crane, &7), "['V', 'R', 'N']");
        assert_eq!(
            view_stack(&crane, &8),
            "['M', 'C', 'V', 'D', 'T', 'L', 'G', 'P']"
        );
        assert_eq!(
            view_stack(&crane, &9),
            "['L', 'M', 'F', 'J', 'N', 'Q', 'W']"
        );

        crane.simulate_one()?;

        assert_eq!(crane.top_crates(), "VQZNJMWTR");

        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let mut crane: super::Crane = std::fs::read_to_string("inputs/day5.test")?.parse()?;

        fn view_stack(crane: &super::Crane, i: &usize) -> String {
            format!("{:?}", crane.boxes.get(i).unwrap())
        }

        crane.simulate_two()?;

        assert_eq!(view_stack(&crane, &1), "['M']");
        assert_eq!(view_stack(&crane, &2), "['C']");
        assert_eq!(view_stack(&crane, &3), "['D', 'N', 'Z', 'P']");

        Ok(())
    }
}
