mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

use clap::Parser;
use colored::*;
use std::time::SystemTime;

const SOLUTIONS: [Day; 8] = [
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    // day09::day09,
    // day10::day10,
    // day11::day11,
    // day12::day12,
    // day13::day13,
    // day14::day14,
    // day15::day15,
    // day16::day16,
    // day17::day17,
    // day18::day18,
    // day19::day19,
    // day20::day20,
    // day21::day21,
    // day22::day22,
    // day23::day23,
    // day24::day24,
    // day25::day25,
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

fn clap_parse_num_range<T>(s: &str, min: T, max: T) -> Result<T, String>
where
    T: std::str::FromStr + std::cmp::PartialOrd + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let val = s.parse::<T>().map_err(|e| format!("{e}"))?;
    if val < min || val > max {
        Err(format!("{val} is not in range [{min}, {max}]"))
    } else {
        Ok(val)
    }
}

fn clap_parse_iterations(s: &str) -> Result<usize, String> {
    clap_parse_num_range(s, 1, 1000)
}

fn clap_parse_days(s: &str) -> Result<usize, String> {
    clap_parse_num_range(s, 1, SOLUTIONS.len())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to run each solution (for more accurate timing)
    #[arg(short, long, default_value_t=1, value_parser=clap_parse_iterations)]
    iterations: usize,

    /// Which solutions to run
    #[arg(short, long, default_values_t = 1..=SOLUTIONS.len(), value_parser = clap_parse_days)]
    days: Vec<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    println!("Running the solutions {} times.", args.iterations);
    println!("{:?}", args.days);
    let mut total_time = 0.;
    println!("| DAY | Duration |      PART 1     |      Part 2     |");
    println!("| :-: | :------: | :-------------: | :-------------: |");
    for i in args.days {
        let f = SOLUTIONS
            .get(i - 1)
            .unwrap_or_else(|| panic!("Day {} not found", i));

        let (t, (p1, p2)) = timeit(f);

        let avg_time = (t + timeit(|| {
            (0..args.iterations - 1).for_each(|_| drop(f()));
        })
        .0) / args.iterations as f64;

        total_time += avg_time;
        println!(
            "| {:3} | {:8} | {:15} | {:15} |",
            i.to_string().yellow(),
            fmt_time(avg_time).green(),
            p1,
            p2
        );
    }
    println!("\nTOTAL TIME: {}", fmt_time(total_time));
    Ok(())
}
