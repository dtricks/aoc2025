#![allow(dead_code)]

const TEST_INPUT1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

const TEST_INPUT2: &str = include_str!("../input.txt");

fn find_largest_joltage(bank: &[u8]) -> u64 {
    let mut max = 0;
    for cursor in 0..bank.len() - 1 {
        for second in cursor + 1..bank.len() {
            let cval = bank[cursor];
            let sval = bank[second];
            let val = digits_to_n(&[cval, sval]);
            if max < val {
                max = val
            }
        }
    }
    max
}

fn digits_to_n(digits: &[u8]) -> u64 {
    let mut mul = 1;
    let mut res = 0;
    for &digit in digits.iter().rev() {
        res += digit as u64 * mul;
        mul *= 10;
    }
    res
}

fn parse_bank(line: &str) -> Vec<u8> {
    line.chars()
        .map(|x| x.to_digit(10).expect("should be only digits") as u8)
        .collect()
}

fn parse_banks(lines: &str) -> Vec<Vec<u8>> {
    lines
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_bank)
        .collect()
}

fn banks_joltage_sum(banks: &str) -> u64 {
    parse_banks(banks)
        .iter()
        .map(|x| find_largest_joltage(x))
        .sum()
}

pub mod part2 {
    use super::*;
    pub fn find_largest_joltage(bank: &[u8]) -> u64 {
        let mut v: Vec<u8> = bank.to_vec();
        let mut lowest_digit = 1;
        while v.len() > 12 {
            let mut iter = v.windows(2);
            // find first ascending pair and delete left
            if let Some(pos) = iter
                .position(|x| x[0] < x[1])
                // or first lowest digit
                .or(v.iter().position(|&x| x == lowest_digit))
            {
                v.remove(pos);
            } else {
                lowest_digit += 1;
            }
        }
        digits_to_n(&v)
    }

    pub fn banks_joltage_sum(banks: &str) -> u64 {
        parse_banks(banks)
            .iter()
            .map(|x| part2::find_largest_joltage(x))
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::part2;
    use super::*;

    #[test]
    fn test_digits_to_n() {
        assert_eq!(digits_to_n(&[1, 2, 3]), 123);
        assert_eq!(digits_to_n(&[7, 8, 3]), 783);
    }

    #[test]
    fn test_find_largest_joltage() {
        assert_eq!(find_largest_joltage(&[7, 8, 3, 1, 2, 3, 4, 5]), 85);
        assert_eq!(find_largest_joltage(&parse_bank("987654321111111")), 98);
        assert_eq!(find_largest_joltage(&parse_bank("811111111111119")), 89);
        assert_eq!(find_largest_joltage(&parse_bank("234234234234278")), 78);
        assert_eq!(find_largest_joltage(&parse_bank("818181911112111")), 92);
    }

    #[test]
    fn test_find_largest_joltage_sum() {
        assert_eq!(banks_joltage_sum(TEST_INPUT1), 357);
        assert_eq!(banks_joltage_sum(TEST_INPUT2), 17613);
    }

    #[test]
    fn test_find_largest_joltage_part2() {
        use part2::*;
        assert_eq!(
            find_largest_joltage(&parse_bank("987654321111111")),
            987654321111
        );
        assert_eq!(
            find_largest_joltage(&parse_bank("811111111111119")),
            811111111119
        );
        assert_eq!(
            find_largest_joltage(&parse_bank("234234234234278")),
            434234234278
        );
        assert_eq!(
            find_largest_joltage(&parse_bank("818181911112111")),
            888911112111
        );
    }

    #[test]
    fn test_find_largest_joltage_sum_part2() {
        assert_eq!(part2::banks_joltage_sum(TEST_INPUT1), 3121910778619);
        assert_eq!(part2::banks_joltage_sum(TEST_INPUT2), 175304218462560);
    }
}
