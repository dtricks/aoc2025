#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

const TEST_INPUT3: &str = "";

const TEST_INPUT2: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct P(u64, u64);

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl std::str::FromStr for P {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(",").expect("should have comma");
        let l = l.parse().expect("should be valid u64");
        let r = r.parse().expect("should be valid u64");
        Ok(Self(l, r))
    }
}

fn parse_input(i: &str) -> Vec<P> {
    i.lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("should be point"))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect(P, P);

impl Rect {
    fn size(&self) -> u64 {
        let l = self.0;
        let r = self.1;
        let x: i64 = ((l.0 as i64) - (r.0 as i64)).abs() + 1;
        let y: i64 = ((l.1 as i64) - (r.1 as i64)).abs() + 1;

        (x * y).try_into().expect("should be valid u64")
    }

    fn contains(&self, p: P) -> bool {
        let range1 = self.0 .0..=self.1 .0;
        let range2 = self.0 .1..=self.1 .1;
        range1.contains(&p.0) && range2.contains(&p.1)
    }
}

fn comb<T>(slice: &[T], k: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    if k == 1 {
        return slice.iter().map(|x| vec![*x]).collect::<Vec<Vec<T>>>();
    }
    if k == slice.len() {
        return vec![slice.to_vec()];
    }

    let mut result = comb(&slice[1..], k - 1)
        .into_iter()
        .map(|x| [&slice[..1], x.as_slice()].concat())
        .collect::<Vec<Vec<T>>>();

    result.extend(comb(&slice[1..], k));
    return result;
}

fn find_max_rect_size(rs: &[P]) -> (Rect, u64) {
    let combinations = comb(&rs, 2);
    let mut max_size = 0;
    let mut max_rect = Rect(P(0, 0), P(0, 0));
    for combination in combinations {
        let l = combination.get(0).expect("should have left");
        let r = combination.get(1).expect("should have right");

        let rect = Rect(*l, *r);
        let size = rect.size();
        if size > max_size {
            max_size = size;
            max_rect = rect;
        }
    }
    (max_rect, max_size)
}

pub mod part2 {
    use super::*;

    pub fn find_max_size_corner(ps: &[P]) -> (Rect, u64) {
        let ps: Vec<_> = ps.iter().chain(ps.iter().take(1)).collect();
        let ps_clone = ps.clone();
        let iter = ps.windows(3);
        let mut max_rect = Rect(P(0, 0), P(0, 0));
        let mut max_size = 0;

        for r in iter {
            let rect = Rect(*r[0], *r[2]);
            let size = rect.size();
            if size > max_size
                && !ps_clone.iter().any(|&&x| rect.contains(x))
                && determinant(r[0], r[1], r[2]) < 0
            {
                max_size = size;
                max_rect = rect;
            }
        }
        (max_rect, max_size)
    }

    /// left or right turn
    pub fn determinant(r0: &P, r1: &P, r2: &P) -> i64 {
        let x1 = r0.0 as i64;
        let y1 = r0.1 as i64;
        let x2 = r1.0 as i64;
        let y2 = r1.1 as i64;
        let x3 = r2.0 as i64;
        let y3 = r2.1 as i64;
        let a = x1 - x2;
        let b = y1 - y2;
        let c = x3 - x2;
        let d = y3 - y2;
        (a * d) - (b * c)
    }

    pub fn debug_print(dim: (u64, u64), ps: &[P]) -> String {
        let mut s = String::new();
        for _i in 0..dim.0 {
            for _j in 0..dim.1 {
                s.push('.');
            }
            s.push('\n');
        }
        for p in ps {}
        s
    }
}

#[cfg(test)]
mod tests {

    use super::part2::*;
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT1),
            vec![
                P(7, 1),
                P(11, 1),
                P(11, 7),
                P(9, 7),
                P(9, 5),
                P(2, 5),
                P(2, 3),
                P(7, 3)
            ]
        );
    }

    #[test]
    fn test_rect_size() {
        assert_eq!(Rect(P(2, 5), P(11, 1)).size(), 50);
        assert_eq!(Rect(P(11, 1), P(2, 5)).size(), 50);
    }

    #[test]
    fn test_max_rect_size() {
        assert_eq!(
            find_max_rect_size(&parse_input(TEST_INPUT1)),
            (Rect(P(11, 1), P(2, 5)), 50)
        );
        assert_eq!(
            find_max_rect_size(&parse_input(TEST_INPUT2)),
            (Rect(P(85880, 83175), P(14109, 16559)), 4781235324)
        );
    }
    #[test]
    fn test_max_corner_size() {
        assert_eq!(find_max_size_corner(&parse_input(TEST_INPUT1)).1, 24);
        assert_eq!(
            find_max_size_corner(&parse_input(TEST_INPUT1)),
            (Rect(P(9, 5), P(2, 3)), 24)
        );
        assert_eq!(find_max_size_corner(&parse_input(TEST_INPUT2)).1, 129076995);
        assert_eq!(
            find_max_size_corner(&parse_input(TEST_INPUT2)),
            // 113677172 too low
            // 129076994 too low
            (Rect(P(9, 5), P(2, 3)), 129076995)
        );
    }
    #[test]
    fn test_debug_print() {
        eprintln!("{}", debug_print((11, 11), &parse_input(TEST_INPUT1)));
        assert_eq!(
            debug_print((11, 11), &parse_input(TEST_INPUT1)),
            "".to_string()
        );
    }
}
