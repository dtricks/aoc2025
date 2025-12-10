#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

const TEST_INPUT1: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

const TEST_INPUT2: &str = include_str!("../input.txt");

fn parse_coord(i: &str) -> C {
    let mut split = i.split(",");
    C(
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

fn parse_coords(i: &str) -> Vec<C> {
    i.lines().map(parse_coord).collect()
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct C(u64, u64, u64);

impl std::fmt::Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C{}", self.0)
    }
}

impl std::fmt::Debug for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("C").field(&self.0).finish()
    }
}

impl C {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self(x, y, z)
    }
}

fn distance_3d(p1: &C, p2: &C) -> f64 {
    let dx = p2.0 as f64 - p1.0 as f64;
    let dy = p2.1 as f64 - p1.1 as f64;
    let dz = p2.2 as f64 - p1.2 as f64;
    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

fn find_min_distance(i: &[C], filter_pairs: &[(C, C)]) -> (C, C) {
    let mut min_distance = f64::INFINITY;
    let mut smallest_pair = (C::new(0, 0, 0), C::new(0, 0, 0));
    for p in comb(&i, 2) {
        let dist = distance_3d(&p[0], &p[1]);
        if dist < min_distance
            && !filter_pairs.contains(&(p[0], p[1]))
            && !filter_pairs.contains(&(p[1], p[0]))
        {
            min_distance = dist;
            smallest_pair = (p[0], p[1])
        }
    }
    smallest_pair
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

fn find_circuits(i: Vec<C>) -> Vec<Vec<(C, C)>> {
    let mut circuits: Vec<Vec<(C, C)>> = vec![];
    let mut filter_pairs = vec![];
    'smallest: for x in 0..11 {
        let (c1, c2) = find_min_distance(&i, &filter_pairs);
        eprintln!("{x}: {c1:?}, {c2:?}");
        let mut inserted = false;
        'circuit: for circuit in circuits.iter_mut() {
            for (cs0, cs1) in circuit.clone() {
                eprintln!("{x}: {c1:?}, {c2:?}, cs0 {cs0:?} cs1 {cs1:?}");
                if cs0 == c1 || cs1 == c1 || cs0 == c2 || cs1 == c2 {
                    eprintln!("{x}: TRUE {c1:?}, {c2:?}, cs0 {cs0:?} cs1 {cs1:?}");
                    let mut circuit_clone = circuit.clone();
                    if !circuit_clone.contains(&(c1, c2)) || !circuit_clone.contains(&(c2, c1)) {
                        eprintln!("{x}: PUSH {c1:?}, {c2:?}, cs0 {cs0:?} cs1 {cs1:?}");
                        circuit_clone.push((c1, c2));
                    }
                    *circuit = circuit_clone;
                    inserted = true;
                    filter_pairs.push((c1, c2));
                    continue 'smallest;
                }
            }
        }
        if !inserted {
            circuits.push(vec![(c1, c2)]);
            filter_pairs.push((c1, c2));
        }
        //eprintln!("{x}: {circuits:?}");
    }
    circuits
}

fn find_circuits_lens(i: &[Vec<(C, C)>]) -> Vec<usize> {
    i.iter().map(|x| x.len()).collect()
}

// pub mod part2 {
//     use super::*;
// }
#[cfg(test)]
mod tests {
    use super::*;
    // #[cfg(test)]
    // use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_coord(TEST_INPUT1.lines().next().unwrap()),
            C::new(162, 817, 812)
        );
        assert_eq!(
            parse_coords(TEST_INPUT1),
            vec![
                C(162, 817, 812),
                C(57, 618, 57),
                C(906, 360, 560),
                C(592, 479, 940),
                C(352, 342, 300),
                C(466, 668, 158),
                C(542, 29, 236),
                C(431, 825, 988),
                C(739, 650, 466),
                C(52, 470, 668),
                C(216, 146, 977),
                C(819, 987, 18),
                C(117, 168, 530),
                C(805, 96, 715),
                C(346, 949, 466),
                C(970, 615, 88),
                C(941, 993, 340),
                C(862, 61, 35),
                C(984, 92, 344),
                C(425, 690, 689)
            ]
        );
    }

    #[test]
    fn test_find_min_distance() {
        assert_eq!(
            find_min_distance(&parse_coords(TEST_INPUT1), &vec![]),
            (C(162, 817, 812), C(425, 690, 689))
        );
        assert_eq!(
            find_min_distance(
                &parse_coords(TEST_INPUT1),
                &vec![(C(162, 817, 812), C(425, 690, 689))]
            ),
            (C(162, 817, 812), C(431, 825, 988))
        );
    }
    #[test]
    fn test_find_circuits_lens() {
        assert_eq!(
            find_circuits_lens(&find_circuits(parse_coords(TEST_INPUT1))),
            vec![5, 4, 2, 1]
        );
    }
    #[test]
    fn test_find_circuits() {
        assert_eq!(
            find_circuits(parse_coords(TEST_INPUT1)),
            vec![vec![(C(162, 817, 812), C(425, 690, 689))]]
        );
    }
}
