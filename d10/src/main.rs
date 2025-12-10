#![allow(dead_code)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        char,
        complete::{line_ending, multispace0, one_of, u64},
    },
    combinator::{eof, map, opt},
    multi::{many1, separated_list1},
    sequence::*,
    IResult, Parser,
};

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

";

const TEST_INPUT3: &str = "";

const TEST_INPUT2: &str = include_str!("../input.txt");

fn parse_machines(input: &str) -> IResult<&str, Vec<(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)>> {
    let machine_with_eol = terminated(parse_machine, opt(line_ending));
    let machines_parser = separated_list1(multispace0, machine_with_eol);
    let machines_with_trailing = terminated(machines_parser, multispace0);
    terminated(machines_with_trailing, eof).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    (
        parse_indicator_lights,
        parse_button_wirings,
        parse_joltage_requirements,
    )
        .parse(input)
}

fn parse_bool(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => unreachable!(),
    }
}

fn parse_indicator_lights(input: &str) -> IResult<&str, Vec<bool>> {
    pair(
        delimited(tag("["), many1(map(one_of(".#"), parse_bool)), tag("]")),
        multispace0,
    )
    .map(|(vec, _)| vec)
    .parse(input)
}

fn parse_button_wirings(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    many1(delimited(
        multispace0,
        delimited(
            char('('),
            separated_list1(char(','), u64), // handles single, pair, or multi!
            char(')'),
        ),
        multispace0,
    ))
    .parse(input)
}

fn parse_joltage_requirements(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        multispace0,
        delimited(
            char('{'),
            separated_list1(char(','), u64), // handles single, pair, or multi!
            char('}'),
        ),
    )
    .parse(input)
}

fn all_fewest_button_presses(i: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> Vec<usize> {
    i.iter().map(fewest_button_presses).collect()
}

fn all_fewest_button_presses_sum(i: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> usize {
    i.iter().map(fewest_button_presses).sum()
}

fn fewest_button_presses(i: &(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)) -> usize {
    let bw = &i.1;
    let mut combinations = 0;
    'outer: loop {
        for button_comb in bw.iter().combinations_with_replacement(combinations) {
            let mut il = i.0.clone();
            for button in button_comb {
                let new_il = apply_button(button, &il);
                il = new_il;
            }
            if !il.iter().any(|&x| x) {
                break 'outer;
            }
        }
        combinations += 1;
    }
    combinations
}

fn apply_button(b: &[u64], il: &[bool]) -> Vec<bool> {
    let mut v = il.to_vec();
    for idx in b {
        v[*idx as usize] = !v[*idx as usize];
    }
    v
}

pub mod part2 {

    use super::*;
    pub fn fewest_button_presses(i: &(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)) -> usize {
        let bw = &i.1;
        let jr = &i.2;
        let mut combinations = 0;
        'outer: loop {
            for button_comb in bw.iter().combinations_with_replacement(combinations) {
                let mut jr_clone = jr.clone();
                for button in button_comb {
                    let underflow = apply_button_inplace(button, &mut jr_clone);
                    if underflow {
                        break;
                    }
                }
                if combinations > 30 || jr_clone == vec![0; jr.len()] {
                    break 'outer;
                }
            }
            combinations += 1;
        }
        dbg!(&combinations);
        combinations
    }

    pub fn apply_button_inplace(b: &[u64], jr: &mut [u64]) -> bool {
        for &idx in b {
            let i = idx as usize;
            if jr[i] as i64 - 1 < 0 {
                return true;
            }
            jr[i] -= 1;
        }
        false
    }

    pub fn all_fewest_button_presses(i: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> Vec<usize> {
        i.iter().map(fewest_button_presses).collect()
    }

    pub fn all_fewest_button_presses_sum(i: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> usize {
        i.iter()
            .map(|x| {
                dbg!(find_optimal_button_sequence(&x.2, &x.1)
                    .unwrap()
                    .iter()
                    .sum::<usize>())
            })
            .sum()
    }
    use std::collections::{HashMap, HashSet, VecDeque};

    type State = Vec<u64>;
    type Buttons = Vec<Vec<u64>>;

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Node {
        state: State,
        cost: usize,
    }

    pub fn find_optimal_button_sequence(target: &[u64], buttons: &Buttons) -> Option<Vec<usize>> {
        println!("DEBUG: target={:?}", target);
        println!("DEBUG: buttons={:?}", buttons);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<State, (State, usize)> = HashMap::new();

        let mut start_state = vec![0u64; target.len()];
        queue.push_back(Node {
            state: start_state.clone(),
            cost: 0,
        });
        visited.insert(start_state.clone());

        let mut count = 0;
        while let Some(node) = queue.pop_front() {
            count += 1;
            if count % 10000 == 0 {
                println!("DEBUG: explored {} states", count);
            }

            let state = &node.state;
            println!("DEBUG: current state={:?}", state);

            if state == target {
                // reconstruct path...
                return Some(vec![]); // placeholder
            }

            for (button_idx, button) in buttons.iter().enumerate() {
                let mut new_state = state.clone();
                let mut valid = true;

                println!("DEBUG: trying button {} = {:?}", button_idx, button);

                for &idx in button {
                    let i = idx as usize;
                    if i >= new_state.len() {
                        println!("DEBUG: invalid index {} >= {}", i, new_state.len());
                        valid = false;
                        break;
                    }
                    if new_state[i].saturating_add(1) > target[i] {
                        println!(
                            "DEBUG: overshoot at {}: {}+1 > {}",
                            i, new_state[i], target[i]
                        );
                        valid = false;
                        break;
                    }
                    new_state[i] += 1;
                }

                println!("DEBUG: new_state={:?}", new_state);

                if valid {
                    if new_state == *target {
                        println!("DEBUG: FOUND TARGET with button {}", button_idx);
                        return Some(vec![button_idx]);
                    }
                    if !visited.contains(&new_state) {
                        visited.insert(new_state.clone());
                        parent.insert(new_state.clone(), (state.clone(), button_idx));
                        queue.push_back(Node {
                            state: new_state,
                            cost: node.cost + 1,
                        });
                    }
                }
            }
        }
        println!("DEBUG: queue empty after {} states", count);
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_machine() {
        let binding = TEST_INPUT1.lines().take(1).collect::<String>();
        let res = parse_indicator_lights(&binding);
        assert_eq!(
            res,
            IResult::Ok((
                "(3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
                vec![false, true, true, false]
            ))
        );
        let res = res.unwrap().0;
        let res = parse_button_wirings(res);
        assert_eq!(
            res,
            IResult::Ok((
                "{3,5,4,7}",
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1]
                ]
            ))
        );
        let res = res.unwrap().0;
        let res = parse_joltage_requirements(res);
        assert_eq!(res, IResult::Ok(("", vec![3, 5, 4, 7])));
        let binding = TEST_INPUT1.lines().take(1).collect::<String>();
        let res = parse_machine(&binding);
        assert_eq!(
            res,
            IResult::Ok((
                "",
                (
                    vec![false, true, true, false],
                    vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1]
                    ],
                    vec![3, 5, 4, 7]
                )
            ))
        );
    }

    #[test]
    fn test_parse_machines() {
        let res = parse_machines(&TEST_INPUT1);
        assert!(res.is_ok());
    }

    #[test]
    fn test_fewest_button_presses() {
        let binding = TEST_INPUT1.lines().take(1).collect::<String>();
        let res = parse_machine(&binding).unwrap().1;
        assert_eq!(fewest_button_presses(&res), 2);
    }

    #[test]
    fn test_all_fewest_button_presses() {
        let res = parse_machines(TEST_INPUT1).unwrap().1;
        assert_eq!(all_fewest_button_presses(&res), vec![2, 3, 2]);
        assert_eq!(all_fewest_button_presses_sum(&res), 7);
        let res = parse_machines(TEST_INPUT2).unwrap().1;
        assert_eq!(all_fewest_button_presses_sum(&res), 507);
        assert_eq!(
            all_fewest_button_presses(&res),
            vec![
                4, 5, 1, 3, 1, 2, 3, 1, 3, 1, 2, 3, 1, 6, 1, 2, 1, 7, 1, 6, 6, 2, 5, 4, 4, 1, 2, 1,
                2, 1, 1, 3, 4, 2, 2, 2, 2, 1, 4, 1, 2, 5, 2, 2, 1, 1, 4, 1, 4, 2, 5, 1, 2, 1, 1, 1,
                4, 3, 1, 1, 2, 4, 1, 4, 2, 5, 3, 1, 4, 3, 2, 1, 2, 7, 4, 2, 2, 1, 2, 2, 1, 3, 3, 2,
                1, 3, 2, 3, 1, 2, 5, 3, 4, 1, 3, 2, 4, 1, 1, 5, 2, 1, 3, 4, 3, 5, 3, 2, 3, 7, 3, 3,
                1, 3, 4, 3, 1, 5, 4, 2, 1, 3, 3, 2, 2, 2, 4, 1, 4, 1, 1, 3, 2, 6, 1, 1, 2, 3, 1, 6,
                2, 2, 2, 3, 3, 2, 2, 2, 3, 1, 1, 2, 2, 6, 3, 2, 2, 1, 4, 5, 4, 2, 3, 1, 4, 5, 1, 2,
                4, 4, 6, 3, 3, 4, 2, 3, 1, 1, 6, 1, 2, 5, 2, 4, 1, 1, 2, 5, 6, 5, 1
            ]
        );
    }
    #[test]
    fn test_all_fewest_button_presses_part2() {
        use part2::*;
        let res = parse_machines(TEST_INPUT1).unwrap().1;
        assert_eq!(all_fewest_button_presses(&res), vec![10, 12, 11]);
        assert_eq!(all_fewest_button_presses_sum(&res), 33);
        let res = parse_machines(TEST_INPUT2).unwrap().1;
        assert_eq!(all_fewest_button_presses_sum(&res), 507);
        // assert_eq!(
        //     all_fewest_button_presses(&res),
        //     vec![
        //         4, 5, 1, 3, 1, 2, 3, 1, 3, 1, 2, 3, 1, 6, 1, 2, 1, 7, 1, 6, 6, 2, 5, 4, 4, 1, 2, 1,
        //         2, 1, 1, 3, 4, 2, 2, 2, 2, 1, 4, 1, 2, 5, 2, 2, 1, 1, 4, 1, 4, 2, 5, 1, 2, 1, 1, 1,
        //         4, 3, 1, 1, 2, 4, 1, 4, 2, 5, 3, 1, 4, 3, 2, 1, 2, 7, 4, 2, 2, 1, 2, 2, 1, 3, 3, 2,
        //         1, 3, 2, 3, 1, 2, 5, 3, 4, 1, 3, 2, 4, 1, 1, 5, 2, 1, 3, 4, 3, 5, 3, 2, 3, 7, 3, 3,
        //         1, 3, 4, 3, 1, 5, 4, 2, 1, 3, 3, 2, 2, 2, 4, 1, 4, 1, 1, 3, 2, 6, 1, 1, 2, 3, 1, 6,
        //         2, 2, 2, 3, 3, 2, 2, 2, 3, 1, 1, 2, 2, 6, 3, 2, 2, 1, 4, 5, 4, 2, 3, 1, 4, 5, 1, 2,
        //         4, 4, 6, 3, 3, 4, 2, 3, 1, 1, 6, 1, 2, 5, 2, 4, 1, 1, 2, 5, 6, 5, 1
        //     ]
        // );
    }
}
