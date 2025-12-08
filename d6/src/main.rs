#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

const TEST_INPUT2: &str = include_str!("../input.txt");

fn parse_cols(input: &str) -> Vec<Vec<u64>> {
    let mut v = vec![];
    'outer: for line in input.lines() {
        let mut v_line = vec![];
        for word in line.split_whitespace() {
            if word == "+" || word == "*" {
                break 'outer;
            }
            let n = word.parse().expect("should be valid u64");
            v_line.push(n);
        }
        v.push(v_line);
    }
    v
}

fn transpose2<T: std::fmt::Debug>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ops {
    Add,
    Mul,
}

impl std::str::FromStr for Ops {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Ops::Add),
            "*" => Ok(Ops::Mul),
            _ => Err(String::from("parse Ops failure")),
        }
    }
}

fn parse_ops(input: &str) -> Vec<Ops> {
    let mut v_line = vec![];
    'outer: for line in input.lines() {
        for word in line.split_whitespace() {
            if !(word == "+" || word == "*") {
                continue 'outer;
            }
            let op = word.parse().expect("should be valid op");
            v_line.push(op);
        }
    }
    v_line
}

fn process_col(nums: &[u64], op: Ops) -> u64 {
    if op == Ops::Add {
        nums.iter().sum()
    } else {
        nums.iter().product()
    }
}

fn process(nums2d: &[Vec<u64>], ops: &[Ops]) -> u64 {
    let mut total = 0;
    for (nums, op) in nums2d.iter().zip(ops) {
        total += process_col(nums, *op);
    }
    total
}

pub mod part2 {
    use super::*;

    pub fn parse_cols(input: &str) -> Vec<Vec<u64>> {
        let mut v = vec![];
        for line in input
            .lines()
            .filter(|x| !(x.contains(&['+', '*']) || x.is_empty()))
        {
            let mut v_line = vec![];
            for c in line.chars() {
                v_line.push(c);
            }
            v.push(v_line);
        }
        let v: Vec<Vec<char>> = transpose2(v);
        let mut res = vec![];
        let mut res_line = vec![];
        for line in v.iter().rev() {
            let s: String = line.iter().filter(|x| !x.is_whitespace()).collect();
            if s.is_empty() {
                res.push(res_line);
                res_line = vec![];
                continue;
            };
            let num = s.parse().expect("should be u64");
            res_line.push(num);
        }
        res.push(res_line);
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_u64_cols() {
        assert_eq!(
            transpose2(parse_cols(TEST_INPUT1)),
            vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314]
            ]
        );
    }

    #[test]
    fn test_parse_ops() {
        use crate::Ops::*;
        assert_eq!(parse_ops(TEST_INPUT1), vec![Mul, Add, Mul, Add]);
    }

    #[test]
    fn test_process() {
        let nums2d = transpose2(parse_cols(TEST_INPUT1));
        let ops = parse_ops(TEST_INPUT1);
        assert_eq!(process(&nums2d, &ops), 4277556);
        let nums2d = transpose2(parse_cols(TEST_INPUT2));
        let ops = parse_ops(TEST_INPUT2);
        assert_eq!(process(&nums2d, &ops), 4364617236318);
    }

    #[test]
    fn test_parse_u64_cols_part2() {
        assert_eq!(
            part2::parse_cols(TEST_INPUT1),
            vec![
                vec![4, 431, 623],
                vec![175, 581, 32],
                vec![8, 248, 369],
                vec![356, 24, 1]
            ]
        );
    }
    #[test]
    fn test_parse_process_part2() {
        assert_eq!(
            process(
                &part2::parse_cols(TEST_INPUT1),
                &parse_ops(TEST_INPUT1)
                    .iter()
                    .cloned()
                    .rev()
                    .collect::<Vec<Ops>>()
            ),
            3263827
        );
        assert_eq!(
            process(
                &part2::parse_cols(TEST_INPUT2),
                &parse_ops(TEST_INPUT2)
                    .iter()
                    .cloned()
                    .rev()
                    .collect::<Vec<Ops>>()
            ),
            9077004354241
        );
    }
}
