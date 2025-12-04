#![allow(dead_code)]

const TEST_INPUT1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

const TEST_INPUT2: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pos {
    Empty,
    Paperroll,
    PaperrollAccessible,
}

fn parse_line(line: &str) -> Vec<Pos> {
    line.chars()
        .filter_map(|char| match char {
            '.' => Some(Pos::Empty),
            '@' => Some(Pos::Paperroll),
            '\n' => None,
            _ => panic!("invalid char"),
        })
        .collect()
}

fn parse_lines(lines: &str) -> Vec<Vec<Pos>> {
    lines.lines().map(parse_line).collect()
}

fn fewer_than_four_neighbors(input: Vec<Vec<Pos>>) -> Vec<Vec<Pos>> {
    use Pos::{Empty as E, Paperroll as P, PaperrollAccessible as PA};
    let mut v = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut v_line = vec![];
        for (x, pos) in line.iter().enumerate() {
            if *pos == P {
                let left = line.get(x.wrapping_sub(1)).unwrap_or(&E).clone();
                let right = line.get(x.wrapping_add(1)).unwrap_or(&E).clone();
                let up = input
                    .get(y.wrapping_sub(1))
                    .unwrap_or(&vec![E; line.len()])
                    .get(x)
                    .unwrap_or(&E)
                    .clone();
                let down = input
                    .get(y + 1)
                    .unwrap_or(&vec![E; line.len()])
                    .get(x)
                    .unwrap_or(&E)
                    .clone();
                let dl = input
                    .get(y + 1)
                    .unwrap_or(&vec![E; line.len()])
                    .get(x.wrapping_sub(1))
                    .unwrap_or(&E)
                    .clone();
                let dr = input
                    .get(y + 1)
                    .unwrap_or(&vec![E; line.len()])
                    .get(x + 1)
                    .unwrap_or(&E)
                    .clone();
                let ur = input
                    .get(y.wrapping_sub(1))
                    .unwrap_or(&vec![E; line.len()])
                    .get(x + 1)
                    .unwrap_or(&E)
                    .clone();
                let ul = input
                    .get(y.wrapping_sub(1))
                    .unwrap_or(&vec![E; line.len()])
                    .get(x.wrapping_sub(1))
                    .unwrap_or(&E)
                    .clone();
                let n = left as i32
                    + right as i32
                    + up as i32
                    + down as i32
                    + ul as i32
                    + ur as i32
                    + dl as i32
                    + dr as i32;
                if n < 4 {
                    v_line.push(PA);
                }
            } else {
                v_line.push(*pos);
            }
        }
        v.push(v_line);
    }
    v
}

fn count_accessible(input: Vec<Vec<Pos>>) -> u64 {
    input.iter().flatten().fold(0, |acc, &x| {
        let x = if x == Pos::PaperrollAccessible { 1 } else { 0 };
        acc + x
    })
}

fn main() {
    println!("Hello, world!");
}

pub mod part2 {
    use super::Pos::{Empty as E, Paperroll as P, PaperrollAccessible as PA};
    use super::*;

    pub fn remove_accessible(input: Vec<Vec<Pos>>) -> (u64, Vec<Vec<Pos>>) {
        let count = count_accessible(input.clone());
        let mut v = vec![];
        for line in input {
            let mut v_line = vec![];
            for pos in line {
                if pos == PA {
                    v_line.push(E);
                } else {
                    v_line.push(pos);
                }
            }
            v.push(v_line);
        }
        println!("{count}");
        (count, v)
    }

    pub fn add_n_removable(input: Vec<Vec<Pos>>) -> u64 {
        let mut v = input.clone();
        let mut removable = 0;
        dbg!(&v);
        loop {
            let (count, v) = remove_accessible(v.clone());
            if count == 0 {
                return removable;
            }
            removable += count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part2;
    use super::*;

    #[test]
    fn test_parse_line() {
        use Pos::{Empty as E, Paperroll as P};
        assert_eq!(parse_line("..@@.@@@@."), vec![E, E, P, P, E, P, P, P, P, E]);
        assert_eq!(parse_line(""), vec![]);
    }

    #[test]
    fn test_parse_lines() {
        use Pos::{Empty as E, Paperroll as P};
        assert_eq!(parse_lines("\n"), vec![vec![]]);
        assert_eq!(
            parse_lines("..@@.@@@@.\n..@@.@@@@."),
            vec![
                vec![E, E, P, P, E, P, P, P, P, E],
                vec![E, E, P, P, E, P, P, P, P, E]
            ]
        );
    }
    #[test]
    fn test_count_accessible() {
        assert_eq!(
            count_accessible(fewer_than_four_neighbors(parse_lines(TEST_INPUT1))),
            13
        );
        assert_eq!(
            count_accessible(fewer_than_four_neighbors(parse_lines(TEST_INPUT2))),
            1451
        );
    }
    #[test]
    fn test_count_removed() {
        use super::part2::*;
        println!("Running part2 test");
        assert_eq!(
            add_n_removable(fewer_than_four_neighbors(parse_lines(TEST_INPUT1))),
            43
        );
        assert_eq!(
            add_n_removable(fewer_than_four_neighbors(parse_lines(TEST_INPUT2))),
            1451
        );
    }
}
