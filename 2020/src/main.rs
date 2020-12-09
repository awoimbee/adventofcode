#![feature(const_str_from_utf8_unchecked)]

mod day01;
mod day06;
mod day07;
mod day08;
mod day09;

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
        t if t < 1e-3 => format!("{}µs", t / 1e-6),
        t if t < 1. => format!("{}ms", t / 1e-3),
        t => format!("{}s", t),
    }
}

fn main() {
    let mut days: Vec<(&str, Day)> = vec![
        ("01", day01::day01),
        ("06", day06::day06),
        ("07", day07::day07),
        ("08", day08::day08),
        ("09", day09::day09),
    ];
    let mut total_time = 0.;
    for (s, f) in days.drain(..) {
        println!("#### DAY{} ####", s);
        let (p1, p2) = f();
        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);
        const NB_RUNS: usize = 1000;
        let avg_time: f64 = (0..NB_RUNS).map(|_| timeit(f)).sum::<f64>() / NB_RUNS as f64;
        println!("-> average over {} runs: {}", NB_RUNS, fmt_time(avg_time));
        total_time += avg_time;
    }
    println!("\nTOTAL TIME: {}", fmt_time(total_time));
}
