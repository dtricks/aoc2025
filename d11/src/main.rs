#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use nom::{
    bytes::tag,
    character::{
        complete::{alpha0, alpha1, newline},
        multispace0,
    },
    combinator::{eof, map},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult, Parser,
};
use pathfinding::prelude::count_paths;

fn main() {
    println!("Hello, world!");
}
pub trait ToOwnedExt
where
    Self: ToOwned,
{
    /// Simply an alias for `.to_owned()`.
    fn o(&self) -> <Self as ToOwned>::Owned {
        self.to_owned()
    }
}
impl<T: ?Sized> ToOwnedExt for T where T: ToOwned {}

const TEST_INPUT1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST_INPUT3: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

const TEST_INPUT2: &str = include_str!("../input.txt");

type Device = (String, Vec<String>);

fn parse_device(input: &str) -> IResult<&str, Device> {
    terminated(
        terminated(map(alpha1, str::to_string), tag(":"))
            .and(many1(preceded(tag(" "), map(alpha1, str::to_string)))),
        newline,
    )
    .parse(input)
}

fn parse_devices(input: &str) -> IResult<&str, Vec<Device>> {
    terminated(many1(parse_device), eof).parse(input)
}

fn successors(start: &Device, others: Vec<Device>) -> Vec<Device> {
    let successors = start.1.clone();
    others
        .iter()
        .filter(|(name, _)| successors.contains(name))
        .cloned()
        .collect()
}

fn count_device_paths(start: &Device, end: &str, others: Vec<Device>) -> usize {
    count_paths(
        start.clone(),
        |x| successors(x, others.clone()),
        |x| x.1.contains(&end.to_owned()),
    )
}

// pub mod part2 {
//     use super::*;
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_device() {
        assert_eq!(
            parse_device(
                "aaa: you hhh
"
            )
            .unwrap()
            .1,
            ("aaa".o(), vec!["you".o(), "hhh".o()])
        );
        assert_eq!(
            parse_devices(
                "aaa: you hhh
bbb: ccc hhh
"
            )
            .unwrap()
            .1,
            vec![
                ("aaa".o(), vec!["you".o(), "hhh".o()]),
                ("bbb".o(), vec!["ccc".o(), "hhh".o()])
            ]
        );
        assert_eq!(parse_devices(TEST_INPUT1).unwrap().1.len(), 10);
        assert_eq!(parse_devices(TEST_INPUT2).unwrap().1.len(), 643);
    }

    #[test]
    fn test_successors() {
        let map = parse_devices(TEST_INPUT1).unwrap().1;
        assert_eq!(
            successors(&map.clone()[0], map),
            vec![
                ("you".o(), vec!["bbb".o(), "ccc".o()]),
                ("hhh".o(), vec!["ccc".o(), "fff".o(), "iii".o()])
            ]
        );
    }

    #[test]
    fn test_pathfinding() {
        let map = parse_devices(TEST_INPUT1).unwrap().1;
        let start_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"you".o())
            .unwrap()
            .clone();
        assert_eq!(count_device_paths(&start_device, "out", map), 5);
        let map = parse_devices(TEST_INPUT2).unwrap().1;
        let start_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"you".o())
            .unwrap()
            .clone();
        assert_eq!(count_device_paths(&start_device, "out", map), 574);
    }
    #[test]
    fn test_pathfinding_part2() {
        let map = parse_devices(TEST_INPUT3).unwrap().1;
        let start_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"svr".o())
            .unwrap()
            .clone();
        let fft_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"fft".o())
            .unwrap()
            .clone();
        let dac_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"dac".o())
            .unwrap()
            .clone();
        let paths = count_device_paths(&start_device, "fft", map.clone())
            * count_device_paths(&fft_device, "dac", map.clone())
            * count_device_paths(&dac_device, "out", map.clone());
        let paths2 = count_device_paths(&start_device, "dac", map.clone())
            * count_device_paths(&dac_device, "fft", map.clone())
            * count_device_paths(&fft_device, "out", map.clone());
        assert_eq!(std::cmp::max(paths, paths2), 2);
        let map = parse_devices(TEST_INPUT2).unwrap().1;
        let start_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"svr".o())
            .unwrap()
            .clone();
        let fft_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"fft".o())
            .unwrap()
            .clone();
        let dac_device: Device = map
            .iter()
            .find(|(name, _x)| name == &"dac".o())
            .unwrap()
            .clone();
        let paths = count_device_paths(&start_device, "fft", map.clone())
            * count_device_paths(&fft_device, "dac", map.clone())
            * count_device_paths(&dac_device, "out", map.clone());
        let paths2 = count_device_paths(&start_device, "dac", map.clone())
            * count_device_paths(&dac_device, "fft", map.clone())
            * count_device_paths(&fft_device, "out", map.clone());
        assert_eq!(std::cmp::max(paths, paths2), 306594217920240);
    }
}
