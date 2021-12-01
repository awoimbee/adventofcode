const INPUT: &str = include_str!("../input/day15.txt");

const ITERATIONS_P1: usize = 2020;
const ITERATIONS_P2: usize = 30000000;

fn parse() -> (Box<[isize; ITERATIONS_P2]>, usize, usize) {
    let numbers: Vec<_> = INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let (&last_number, rest) = numbers.split_last().unwrap();
    let mut number_timestamps = box [-1; ITERATIONS_P2];
    for (n, i) in rest.iter().enumerate() {
        number_timestamps[*i] = n as isize + 1;
    }
    (number_timestamps, numbers.len(), last_number)
}

fn run_until(
    round: &mut usize,
    nb_timestamps: &mut Box<[isize; ITERATIONS_P2]>,
    last_nb: &mut usize,
    max_iter: usize,
) {
    while *round < max_iter {
        let new_number = if nb_timestamps[*last_nb] != -1 {
            *round - nb_timestamps[*last_nb] as usize
        } else {
            0
        };
        nb_timestamps[*last_nb] = *round as isize;
        *last_nb = new_number;
        *round += 1;
    }
}

fn solve() -> (usize, usize) {
    let (mut number_timestamps, mut round, mut last_number) = parse();

    run_until(
        &mut round,
        &mut number_timestamps,
        &mut last_number,
        ITERATIONS_P1,
    );
    let p1 = last_number;
    run_until(
        &mut round,
        &mut number_timestamps,
        &mut last_number,
        ITERATIONS_P2,
    );
    let p2 = last_number;
    (p1, p2)
}

pub fn day15() -> (String, String) {
    let (p1, p2) = solve();

    (p1.to_string(), p2.to_string())
}
