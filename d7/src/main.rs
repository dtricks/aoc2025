#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

const TEST_INPUT2: &str = include_str!("../input.txt");

const TEST_INPUT3: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Manifold {
    E,
    Start,
    S,
    US(u64),
    B,
}

fn parse_line(line: &str) -> Vec<Manifold> {
    use Manifold::*;
    line.chars()
        .map(|c| match c {
            '.' => E,
            'S' => Start,
            '^' => S,
            '|' => B,
            _ => panic!("should be manifold character"),
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Manifold>> {
    input
        .lines()
        .map(parse_line)
        .filter(|x| !x.iter().all(|c| *c == Manifold::E))
        .collect()
}

fn beam(input: &[Vec<Manifold>]) -> u64 {
    use Manifold::*;
    let len = input[0].len();
    let start_index = input[0]
        .iter()
        .position(|x| *x == Start)
        .expect("first line should have Start");
    let mut beam_idxs = vec![start_index];
    let mut total = 0;
    for line in input.iter().skip(1) {
        for (idx, item) in line.iter().enumerate() {
            if beam_idxs.contains(&idx) {
                match item {
                    E => (),
                    S => {
                        // remove idx from beam_idxs
                        beam_idxs = beam_idxs.iter().copied().filter(|x| *x != idx).collect();
                        // add idx+1 and idx-1 to beam_idxs
                        if !(idx == 0) && !(idx == len) {
                            beam_idxs.push(idx - 1);
                            beam_idxs.push(idx + 1);
                        }
                        total += 1;
                    }
                    _ => (),
                }
            }
            beam_idxs.sort_unstable();
            beam_idxs.dedup();
        }
    }
    total
}

pub mod part2 {
    use super::*;
    pub fn beam_paths(input: &[Vec<Manifold>]) -> Vec<Vec<Manifold>> {
        use Manifold::*;
        let len = input[0].len();
        let start_index = input[0]
            .iter()
            .position(|x| *x == Start)
            .expect("first line should have Start");
        let mut beam_idxs = vec![start_index];
        let mut out = vec![];
        out.push(input[0].clone());
        let mut input_copy: Vec<Vec<Manifold>> = input.to_vec();
        for line in input_copy.iter_mut().skip(1) {
            for (idx, item) in line.iter_mut().enumerate() {
                if beam_idxs.contains(&idx) {
                    match item {
                        S => {
                            // remove idx from beam_idxs
                            beam_idxs = beam_idxs.iter().copied().filter(|x| *x != idx).collect();
                            // add idx+1 and idx-1 to beam_idxs
                            if !(idx == 0) && !(idx == len) {
                                beam_idxs.push(idx - 1);
                                beam_idxs.push(idx + 1);
                            }
                            *item = US(1);
                        }
                        US(n) => {
                            beam_idxs = beam_idxs.iter().copied().filter(|x| *x != idx).collect();
                            // add idx+1 and idx-1 to beam_idxs
                            if !(idx == 0) && !(idx == len) {
                                beam_idxs.push(idx - 1);
                                beam_idxs.push(idx + 1);
                            }
                            *item = US(*n + 1);
                        }
                        _ => (),
                    }
                }
            }
            beam_idxs.sort_unstable();
            beam_idxs.dedup();

            let out_line: &mut Vec<Manifold> = &mut line.clone();
            for beam in &beam_idxs {
                out_line[*beam] = B;
            }
            out.push(out_line.to_vec());
        }
        out
    }

    pub fn count_paths(input: &[Vec<Manifold>]) -> u64 {
        use Manifold::*;
        let v: Vec<Manifold> = input.iter().flatten().copied().collect();
        v.windows(3)
            .filter(|&b| b[0] == B && b[1] == S && b[2] == B)
            .count()
            .try_into()
            .unwrap()
    }

    enum Step {
        L,
        R,
    }

    // pub fn get_path_for_end_beam(input: &[Vec<Manifold>]) -> Vec<Step> {
    //     use Manifold::*;
    //     let mut steps = vec![];
    //     let mut iter = input.iter().peekable();
    //     while let Some(line) = iter.next() {
    //         for (idx, beam) in line.iter().enumerate() {
    //             if *beam != S {
    //                 continue;
    //             }
    //             if let Some(&peek) = iter.peek() {}
    //         }
    //     }
    //     steps
    // }
}

#[cfg(test)]
mod tests {
    use super::Manifold::*;
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(TEST_INPUT1),
            vec![
                vec![E, E, E, E, E, E, E, Start, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, S, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, S, E, S, E, E, E, E, E, E],
                vec![E, E, E, E, E, S, E, S, E, S, E, E, E, E, E],
                vec![E, E, E, E, S, E, S, E, E, E, S, E, E, E, E],
                vec![E, E, E, S, E, S, E, E, E, S, E, S, E, E, E],
                vec![E, E, S, E, E, E, S, E, E, E, E, E, S, E, E],
                vec![E, S, E, S, E, S, E, S, E, S, E, E, E, S, E]
            ]
        );
    }

    #[test]
    fn test_count_splitters() {
        assert_eq!(beam(&parse(TEST_INPUT1)), 21);
        assert_eq!(beam(&parse(TEST_INPUT2)), 1660);
    }

    #[test]
    fn test_beam_paths() {
        let bp = part2::beam_paths(&parse(TEST_INPUT1));
        assert_eq!(bp.iter().flatten().filter(|&x| *x == S).count(), 1);
        assert_eq!(bp.iter().flatten().filter(|&x| *x == US(1)).count(), 21);
        // assert_eq!(
        //     part2::beam_paths(&parse(TEST_INPUT1)),
        //     // .......S.......
        //     // ......|^|......
        //     // .....|^|^|.....
        //     // ....|^|^|^|....
        //     // ...|^|^|||^|...
        //     // ..|^|^|||^|^|..
        //     // .|^|||^||.||^|.
        //     // |^|^|^|^|^|||^|
        //     vec![
        //         vec![E, E, E, E, E, E, E, Start, E, E, E, E, E, E, E],
        //         vec![E, E, E, E, E, E, B, S, B, E, E, E, E, E, E],
        //         vec![E, E, E, E, E, B, S, B, S, B, E, E, E, E, E],
        //         vec![E, E, E, E, B, S, B, S, B, S, B, E, E, E, E],
        //         vec![E, E, E, B, S, B, S, B, B, B, S, B, E, E, E],
        //         vec![E, E, B, S, B, S, B, B, B, S, B, S, B, E, E],
        //         vec![E, B, S, B, B, B, S, B, B, E, B, B, S, B, E],
        //         vec![B, S, B, S, B, S, B, S, B, S, B, B, B, S, B],
        //     ]
        // );
        let bp = part2::beam_paths(&parse(TEST_INPUT3));
        assert_eq!(bp.iter().flatten().filter(|&x| *x == S).count(), 0);
        assert_eq!(bp.iter().flatten().filter(|&x| *x == US(1)).count(), 1);
        assert_eq!(
            part2::beam_paths(&parse(TEST_INPUT3)),
            // .......S.......
            // ......|^|......
            // .....|^|^|.....
            vec![
                vec![E, E, E, E, E, E, E, Start, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, B, S, B, E, E, E, E, E, E],
                vec![E, E, E, E, E, B, S, B, S, B, E, E, E, E, E],
            ]
        );
    }
    #[test]
    fn test_count_paths() {
        assert_eq!(
            part2::count_paths(&part2::beam_paths(&parse(TEST_INPUT3))),
            4
        );
        assert_eq!(
            part2::count_paths(&part2::beam_paths(&parse(TEST_INPUT2))),
            //3081 too low
            //3518 too low
            3519
        );
    }
}
