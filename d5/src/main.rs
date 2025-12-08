#![allow(dead_code)]

use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

const TEST_INPUT3: &str = "2-10
3-7
7-20
33-37
36-40

1
5
8
11
17
32
";

const TEST_INPUT2: &str = include_str!("../input.txt");

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .map(|x| {
            let (l, r) = x.split_once("-").expect("should be split by '-'");
            let l = l.parse().expect("should be valid u64");
            let r = r.parse().expect("should be valid u64");
            l..=r
        })
        .collect()
}

fn parse_u64s(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|x| x.parse().expect("should be u64"))
        .collect()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ingredients) = input
        .split_once("\n\n")
        .expect("should have full newline separator");
    (parse_ranges(ranges.trim()), parse_u64s(ingredients.trim()))
}

fn in_ranges(ranges: &[RangeInclusive<u64>], ing: &[u64]) -> u64 {
    ing.iter().fold(0, |acc, i| {
        acc + ranges.iter().any(|x| x.contains(i)) as u64
    })
}

pub mod part2 {
    use super::*;

    pub fn in_ranges(ranges: &[RangeInclusive<u64>]) -> u64 {
        ranges.iter().fold(0, |acc, range| {
            let mut res = acc;
            res += range.end() + 1;
            res -= range.start();
            res
        })
    }

    pub fn remove_overlaps(
        ranges: &[RangeInclusive<u64>],
        counter: u8,
    ) -> Vec<RangeInclusive<u64>> {
        let mut v = vec![];
        let mut ranges = ranges.to_vec();
        v.push(ranges.get(0).expect("ranges should have entries").clone());
        'outer: for range in ranges.iter_mut().skip(1) {
            assert!(range.start() <= range.end());
            for check_range in v.iter_mut() {
                if check_range.contains(range.start()) && check_range.contains(range.end()) {
                    continue 'outer;
                }
                if check_range.contains(range.start()) {
                    if range.end() > check_range.start() {
                        *check_range = *check_range.start()..=*range.end();
                    }
                }
                if check_range.contains(range.end()) {
                    if range.start() < check_range.start() {
                        *check_range = *range.start()..=*check_range.end();
                    }
                }
                if check_range.contains(range.start()) && check_range.contains(range.end()) {
                    continue 'outer;
                }
            }
            v.push(range.clone());
        }
        if counter > 10 {
            panic!("stack frames")
        }
        if has_overlaps(&v) {
            v = remove_overlaps(&v, counter + 1);
        }
        v
    }

    pub fn has_overlaps(ranges: &[RangeInclusive<u64>]) -> bool {
        let mut ranges: Vec<_> = ranges.to_vec();
        ranges.sort_unstable_by(|a, b| a.end().cmp(b.start()));
        let b = ranges.is_sorted_by(|a, b| a.end() < b.start());
        let b2 = ranges.is_sorted_by(|a, b| a.end() < b.start());
        let b3 = ranges.is_sorted_by(|a, b| a.end() < b.start());
        dbg!(&ranges, b, b2, b3);
        !(b || b2 || b3)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT1),
            (
                vec![3..=5, 10..=14, 16..=20, 12..=18],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
    }

    #[test]
    fn test_in_ranges() {
        let (ranges, ing) = parse_input(TEST_INPUT1);
        assert_eq!(in_ranges(&ranges, &ing), 3);
        let (ranges, ing) = parse_input(TEST_INPUT2);
        assert_eq!(in_ranges(&ranges, &ing), 789);
    }

    #[test]
    fn test_in_ranges_part2() {
        use super::part2::*;
        let (ranges, _ing) = parse_input(TEST_INPUT1);
        assert_eq!(in_ranges(&remove_overlaps(&ranges, 0)), 14);
        let (ranges, _ing) = parse_input(TEST_INPUT3);
        assert_eq!(in_ranges(&remove_overlaps(&ranges, 0)), 27);
        let (ranges, _ing) = parse_input(TEST_INPUT2);
        //wrong 220014224367346
        assert_eq!(in_ranges(&remove_overlaps(&ranges, 0)), 343329651880509);
    }

    #[test]
    fn test_has_overlaps_part2() {
        use super::part2::*;
        let v = vec![1..=200, 90..=201, 0..=2, 301..=301];
        assert_eq!(has_overlaps(&v), true);
        assert_eq!(has_overlaps(&remove_overlaps(&v, 0)), false);
    }
}
