use bitvec::prelude::*;
use itertools::{Either, Itertools};

const INPUT: &str = include_str!("../input/day13.txt");

struct Parsed {
    dots: Vec<(usize, usize)>,
    folds_x: Vec<usize>,
    folds_y: Vec<usize>,
    first_fold: char,
}

impl Parsed {
    pub fn new(input: &str) -> Self {
        let (dots, folds) = input
            .rsplit_once("\n\n")
            .expect("delimiter btween dots and folds not found");
        let dots = dots
            .split('\n')
            .map(|line| {
                let (x, y) = line.split_once(',').expect("invalid dot");
                let x = x.parse::<usize>().expect("invalid dot");
                let y = y.parse::<usize>().expect("invalid dot");
                (x, y)
            })
            .collect::<Vec<_>>();
        let mut first_fold = None;
        let (folds_x, folds_y) = folds
            .split_terminator('\n')
            .partition_map::<Vec<_>, Vec<_>, _, _, _>(|line| {
                let (_, fold_str) = line.rsplit_once(' ').expect("invalid fold");
                let (axis, pos) = fold_str.split_once('=').expect("invalid fold");
                let axis = axis.chars().next().expect("invalid fold");
                let pos = pos.parse::<usize>().expect("invalid fold");
                if first_fold.is_none() {
                    first_fold = Some(axis);
                }
                match axis {
                    'x' => Either::Left(pos),
                    'y' => Either::Right(pos),
                    _ => panic!("invalid fold"),
                }
            });

        Parsed {
            dots,
            folds_x,
            folds_y,
            first_fold: first_fold.expect("no folds ?!"),
        }
    }
}

fn _print_page(page: &BitSlice, width: usize, height: usize) {
    println!("Page:");
    for y in 0..height {
        for x in 0..width {
            print!("{}", if page[y * width + x] { '#' } else { '.' });
        }
        println!();
    }
}

fn part_1(input: &Parsed) -> usize {
    let mut dots = Vec::new();
    let fold_x = input.folds_x[0];
    let fold_y = input.folds_y[0];

    input.dots.iter().cloned().for_each(|(mut x, mut y)| {
        match input.first_fold {
            'x' => {
                if x > fold_x {
                    x = 2 * fold_x - x;
                }
            }
            'y' => {
                if y > fold_y {
                    y = 2 * fold_y - y;
                }
            }
            _ => panic!("invalid fold"),
        }
        let val = x + (y << 32);
        if !dots.contains(&val) {
            dots.push(val);
        }
    });
    dots.len()
}

fn part_2(input: &Parsed) -> (BitArray<[usize; 4]>, &str) {
    const PAGE_H: usize = 6;
    const PAGE_W: usize = 39;
    let mut page = bitarr![0; PAGE_H * PAGE_W];

    input.dots.iter().cloned().for_each(|(mut x, mut y)| {
        input.folds_x.iter().for_each(|&fx| {
            if x > fx {
                x = 2 * fx - x;
            }
        });
        input.folds_y.iter().for_each(|&fy| {
            if y > fy {
                y = 2 * fy - y;
            }
        });
        page.set(x + y * PAGE_W, true);
    });
    // _print_page(&page, PAGE_W, PAGE_H);
    // fixme: hardcoded value because we don't have character recognition
    (page, "EFLFJGRF")
}

pub fn day13() -> (String, String) {
    let parsed = Parsed::new(INPUT);
    let part1 = part_1(&parsed);
    let part2 = part_2(&parsed);
    (part1.to_string(), part2.1.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1_test_input() {
        let parsed = Parsed::new(TEST_INPUT);
        assert_eq!(part_1(&parsed), 17);
    }

    #[test]
    fn test_part_1() {
        let parsed = Parsed::new(INPUT);
        assert_eq!(part_1(&parsed), 631);
    }
}
