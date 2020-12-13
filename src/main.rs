#![feature(const_str_from_utf8_unchecked)]
#![feature(const_generics)]

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
    const NB_RUNS: usize = 1;
    const SOLUTIONS: [(&'static str, Day); 13] = [
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
    ];

    println!("Running the solutions {} times.", NB_RUNS);

    let mut total_time = 0.;
    for (s, f) in SOLUTIONS.iter() {
        let (p1, p2) = f();
        let avg_time: f64 = (0..NB_RUNS).map(|_| timeit(*f)).sum::<f64>() / NB_RUNS as f64;
        total_time += avg_time;
        println!(
            "{title:} ({duration:})\nPart 1: {p1:}\nPart 2: {p2:}",
            title = format!("#### DAY{} ####", s).yellow(),
            duration = fmt_time(avg_time).green(),
            p1 = p1,
            p2 = p2,
        );
    }
    println!("\nTOTAL TIME: {}", fmt_time(total_time));
}
