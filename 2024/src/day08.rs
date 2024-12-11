use core::str;
use std::ops::{Add, Sub};

use log::debug;

const INPUT: &str = include_str!("../input/day08.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point<T>(T, T);

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[allow(dead_code)]
fn map_as_str(map: &[Vec<u8>]) -> String {
    map.iter()
        .map(|line| str::from_utf8(line).unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("\n")
}

fn solve(input: &str) -> (u32, u32) {
    let width = input.as_bytes().iter().position(|&c| c == b'\n').unwrap() as i32;
    let height = (input.len() + 1) as i32 / (width + 1);

    let mut map_2d_p1 = input
        .lines()
        .map(|s| s.as_bytes().to_owned())
        .collect::<Vec<_>>();

    let mut map_2d_p2 = input
        .lines()
        .map(|s| s.as_bytes().to_owned())
        .collect::<Vec<_>>();

    let mut flat_nodes = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c != b'.' && c != b'\n' {
                Some((c, i as i32))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    flat_nodes.sort_by(|a, b| a.0.cmp(&b.0));

    let mut last_node = 0;
    let mut grouped_nodes: Vec<Vec<Point<i32>>> = Vec::with_capacity(30);
    for n in flat_nodes {
        let pt = Point(n.1 % (width + 1), n.1 / (width + 1));
        if n.0 == last_node {
            grouped_nodes.last_mut().unwrap().push(pt);
        } else {
            grouped_nodes.push(vec![pt]);
        }
        last_node = n.0;
    }
    debug!("Grouped nodes: {:?}", grouped_nodes);

    let mut p1 = 0;
    let mut p2 = 0;
    for node_group in grouped_nodes {
        let mut it = node_group.iter();

        while let Some(&n0) = it.next() {
            for &n1 in it.clone() {
                let delta = n0 - n1;
                debug_assert_eq!(n0 - delta, n1);
                debug_assert_eq!(n1 + delta, n0);

                let antinodes_p1 = [n0 + delta, n1 - delta];
                for &a in antinodes_p1.iter() {
                    if (0..width).contains(&a.0) && (0..height).contains(&a.1) {
                        let loc = &mut map_2d_p1[a.1 as usize][a.0 as usize];
                        if *loc != b'#' {
                            p1 += 1;
                            *loc = b'#';
                        }
                    }
                }

                let mut antinodes_p2 = [n0, n1];
                let mut out_of_bounds = false;
                while !out_of_bounds {
                    out_of_bounds = true;
                    for a in antinodes_p2 {
                        if (0..width).contains(&a.0) && (0..height).contains(&a.1) {
                            out_of_bounds = false;
                            let loc = &mut map_2d_p2[a.1 as usize][a.0 as usize];
                            if *loc != b'#' {
                                p2 += 1;
                                *loc = b'#';
                            }
                        }
                    }
                    antinodes_p2[0] = antinodes_p2[0] + delta;
                    antinodes_p2[1] = antinodes_p2[1] - delta;
                }
            }
        }
    }

    (p1, p2)
}

pub fn day08() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part_1() {
        init();
        assert_eq!(solve(TEST_INPUT).0, 14);
    }

    #[test]
    fn test_part_2() {
        init();
        assert_eq!(solve(TEST_INPUT).1, 34);
    }
}

// 693 too low for p2

// got
// ##....#....#
// .#.#....0...
// ..#.#0....#.
// ..##...0....
// ....#....#..
// .#...##....#
// ...#..#.....
// #....#.#....
// ..#.....A...
// ....#....A..
// .#........#.
// ............

// target
// ##....#....#
// .#.#....0...
// ..#.#0....#.
// ..##...0....
// ....0....#..
// .#...#A....#
// ...#..#.....
// #....#.#....
// ..#.....A...
// ....#....A..
// .#........#.
// ...#......##
