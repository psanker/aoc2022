use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;

const MAX_DISK_SIZE: usize = 70_000_000;

type FileStat = (String, Option<usize>);
type FileTree = BTreeMap<OsString, Vec<FileStat>>;

enum Instruction {
    GoToRoot,
    GoDown(String),
    GoUp,
    AddFile(FileStat),
    NoOp,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmd_elements: Vec<_> = s.split(" ").collect();

        if s.starts_with("$") {
            _ = cmd_elements.drain(..1); // Get rid of the $

            if cmd_elements[0] == "ls" {
                return Ok(Self::NoOp);
            } else if cmd_elements[0] == "cd" {
                if cmd_elements[1] == ".." {
                    return Ok(Self::GoUp);
                } else if cmd_elements[1] == "/" {
                    return Ok(Self::GoToRoot);
                } else {
                    return Ok(Self::GoDown(cmd_elements[1].to_owned()));
                }
            }
        }

        if cmd_elements[0] == "dir" {
            return Ok(Self::AddFile((cmd_elements[1].to_owned(), None)));
        }

        // All that's left are actual files
        let f_size: usize = cmd_elements[0].parse()?;

        Ok(Self::AddFile((cmd_elements[1].to_owned(), Some(f_size))))
    }
}

struct State {
    pwd: PathBuf,
    tree: FileTree,
}

impl State {
    fn new() -> Self {
        Self {
            pwd: PathBuf::new(),
            tree: BTreeMap::new(),
        }
    }

    fn eval(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::GoToRoot => self.pwd = PathBuf::from("/"),
            Instruction::GoDown(dir) => {
                self.pwd.push(dir);
            }
            Instruction::GoUp => {
                self.pwd.pop();
            }
            Instruction::AddFile(fs) => {
                let key = self.pwd.clone().into_os_string();

                self.tree
                    .entry(key)
                    .and_modify(|vec| vec.push(fs.clone()))
                    .or_insert(vec![fs]);
            }
            Instruction::NoOp => {}
        }
    }
}

fn dir_size(tree: &FileTree, dir: &OsStr) -> usize {
    tree.get(dir.clone()).map_or(0, |v| {
        v.iter()
            .map(|(name, size)| {
                if size.is_none() {
                    let mut subdir = PathBuf::from(dir);
                    subdir.push(name.clone());

                    dir_size(tree, subdir.as_os_str())
                } else {
                    size.unwrap()
                }
            })
            .sum()
    })
}

fn dir_sizes(tree: &FileTree) -> BTreeMap<&OsStr, usize> {
    tree.iter()
        .map(|(k, _)| (k.as_os_str(), dir_size(tree, k)))
        .collect::<BTreeMap<_, _>>()
}

fn sum_smallest_dirs(tree: &FileTree, threshold: usize) -> usize {
    dir_sizes(tree)
        .iter()
        .filter(|(_, size)| **size < threshold)
        .map(|(_, size)| size)
        .sum()
}

fn smallest_dir_to_delete(tree: &FileTree, space_avail: usize) -> Option<usize> {
    let dirs = dir_sizes(tree);
    let current_size = dirs
        .get(OsString::from("/").as_os_str())
        .expect("Root must be present");

    let frees = dirs
        .iter()
        .map(|(k, size)| (k, MAX_DISK_SIZE - (*current_size - *size)))
        .filter(|(_, size)| *size >= space_avail)
        .collect::<BTreeMap<_, _>>();

    let min_free = frees
        .iter()
        .map(|(_, v)| v)
        .min()
        .expect("There should be a minimum");
    let min_dir = frees
        .iter()
        .filter(|(_, v)| **v == *min_free)
        .map(|(k, _)| k)
        .nth(0)
        .expect("The min dir should exist");

    dirs.get(*min_dir).copied()
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day7.prod")?;
    let mut lines = content.lines();
    let mut state = State::new();

    while let Some(line) = lines.next() {
        let instruction: Instruction = line.parse()?;
        state.eval(instruction);
    }

    println!("Part 1: {}", sum_smallest_dirs(&state.tree, 100_000));
    println!("Part 2: {:?}", smallest_dir_to_delete(&state.tree, 30_000_000));

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use crate::{smallest_dir_to_delete, sum_smallest_dirs, Instruction, State};

    #[test]
    fn test_one() -> Result<()> {
        let content = std::fs::read_to_string("inputs/day7.test")?;
        let mut lines = content.lines();
        let mut state = State::new();

        while let Some(line) = lines.next() {
            let instruction: Instruction = line.parse()?;
            state.eval(instruction);
        }

        assert_eq!(95437, sum_smallest_dirs(&state.tree, 100_000));

        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let content = std::fs::read_to_string("inputs/day7.test")?;

        let mut lines = content.lines();
        let mut state = State::new();

        while let Some(line) = lines.next() {
            let instruction: Instruction = line.parse()?;
            state.eval(instruction);
        }

        assert_eq!(
            Some(24933642),
            smallest_dir_to_delete(&state.tree, 30_000_000)
        );

        Ok(())
    }
}
