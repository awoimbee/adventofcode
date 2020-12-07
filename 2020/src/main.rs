#![feature(const_str_from_utf8_unchecked)]

mod day01;
mod day06;
mod day07;

use std::time::SystemTime;

fn time(s: &str, f: fn() -> ()) {
	println!("#### DAY{} ####", s);
    let t0 = SystemTime::now();
    f();
    let t1 = SystemTime::now();
    let duration = t1.duration_since(t0).unwrap().as_secs_f64();
    let duration_str = if duration < 1e-3 {
        format!("{}µs", duration / 1e-6)
    } else if duration < 1. {
        format!("{}ms", duration / 1e-3)
    } else {
        format!("{}s", duration)
    };
    println!("-> {}", duration_str);
}

fn main() {
    time("01", day01::day01);
    time("06", day06::day06);
    time("07", day07::day07);
}
