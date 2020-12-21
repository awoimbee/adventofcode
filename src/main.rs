#![feature(const_str_from_utf8_unchecked)]
#![feature(box_syntax)]
#![feature(const_mut_refs)]
#![feature(destructuring_assignment)]
#![feature(panic_info_message)]
#![feature(generators, generator_trait)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

use clap::{App, Arg};
use colored::*;
use std::time::SystemTime;

const SOLUTIONS: [Day; 19] = [
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
    day10::day10,
    day11::day11,
    day12::day12,
    day13::day13,
    day14::day14,
    day15::day15,
    day16::day16,
    day17::day17,
    day18::day18,
    day19::day19,
];

type Day = fn() -> (String, String);

fn timeit<F, E>(f: F) -> (f64, E)
where
    F: Fn() -> E,
    E: Sized,
{
    let t0 = SystemTime::now();
    let res = f();
    let t1 = SystemTime::now();
    (t1.duration_since(t0).unwrap().as_secs_f64(), res)
}

fn fmt_time(t: f64) -> String {
    match t {
        t if t < 1e-3 => format!("{:.2}µs", t / 1e-6),
        t if t < 1. => format!("{:.2}ms", t / 1e-3),
        t => format!("{:.2}s", t),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("advent of code 2020")
        .version("1.0")
        .author("Arthur Woimbée <arthur.woimbee@gmail.com>")
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iter")
                .takes_value(true)
                .help("Number of times to run each day"),
        )
        .arg(
            Arg::with_name("days")
                .short("d")
                .long("days")
                .multiple(true)
                .takes_value(true)
                .help("list of days to run"),
        )
        .get_matches();

    let nb_runs: usize = matches
        .value_of("iterations")
        .map(|i| i.parse().expect("Not a valid number of runs"))
        .unwrap_or(1);
    let what_to_run: Vec<usize> = matches
        .values_of("days")
        .map(|values| {
            values
                .map(|v| {
                    let nb = v.parse::<usize>().expect("invalid number") - 1;
                    assert!(
                        (0..SOLUTIONS.len()).contains(&nb),
                        format!(
                            "Invalid day: {} (valid range is 1 to {})",
                            v,
                            SOLUTIONS.len()
                        )
                    );
                    nb
                })
                .collect()
        })
        .unwrap_or_else(|| (0..SOLUTIONS.len()).collect());

    println!("Running the solutions {} times.", nb_runs);

    let mut total_time = 0.;
    println!("| DAY | Duration |      PART 1     |      Part 2     |");
    println!("| :-: | :------: | :-------------: | :-------------: |");
    for i in what_to_run {
        let f = SOLUTIONS[i];

        let (t, (p1, p2)) = timeit(f);

        let avg_time = (t + timeit(|| {
            (0..nb_runs - 1).for_each(|_| drop(f()));
        })
        .0) / nb_runs as f64;

        total_time += avg_time;
        println!(
            "| {:3} | {:8} | {:15} | {:15} |",
            (i + 1).to_string().yellow(),
            fmt_time(avg_time).green(),
            p1,
            p2
        );
    }
    println!("\nTOTAL TIME: {}", fmt_time(total_time));
    Ok(())
}
