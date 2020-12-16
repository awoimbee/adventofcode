#![feature(const_str_from_utf8_unchecked)]

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

use colored::*;
use std::time::SystemTime;

type Day = fn() -> (String, String);

fn timeit(f: Day) -> f64 {
    let t0 = SystemTime::now();
    f();
    let t1 = SystemTime::now();
    t1.duration_since(t0).unwrap().as_secs_f64()
}

fn fmt_time(t: f64) -> String {
    match t {
        t if t < 1e-3 => format!("{:.2}µs", t / 1e-6),
        t if t < 1. => format!("{:.2}ms", t / 1e-3),
        t => format!("{:.2}s", t),
    }
}

fn main() {
    let argv: Vec<_> = std::env::args().collect();
    let nb_runs = argv
        .get(1)
        .map(|s| s.parse::<usize>().expect("Not a valid number"))
        .unwrap_or(1);
    const SOLUTIONS: [(&'static str, Day); 15] = [
        ("01", day01::day01),
        ("02", day02::day02),
        ("03", day03::day03),
        ("04", day04::day04),
        ("05", day05::day05),
        ("06", day06::day06),
        ("07", day07::day07),
        ("08", day08::day08),
        ("09", day09::day09),
        ("10", day10::day10),
        ("11", day11::day11),
        ("12", day12::day12),
        ("13", day13::day13),
        ("14", day14::day14),
        ("15", day15::day15),
    ];

    println!("Running the solutions {} times.", nb_runs);

    let mut total_time = 0.;
    println!("| DAY | Duration |      PART 1     |      Part 2     |");
    println!("| :-: | :------: | :-------------: | :-------------: |");
    for (s, f) in SOLUTIONS.iter() {
        let (p1, p2) = f();
        let avg_time: f64 = (0..nb_runs).map(|_| timeit(*f)).sum::<f64>() / nb_runs as f64;
        total_time += avg_time;
        println!(
            "| {:3} | {:8} | {:15} | {:15} |",
            s.yellow(),
            fmt_time(avg_time).green(),
            p1,
            p2
        );
    }
    println!("\nTOTAL TIME: {}", fmt_time(total_time));
}
