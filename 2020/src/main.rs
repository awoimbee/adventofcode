mod day01;
mod day06;

use std::time::SystemTime;

fn time(s: &str, f: fn() -> ()) {
	let t0 = SystemTime::now();
	f();
	let t1 = SystemTime::now();
	let duration = t1.duration_since(t0).unwrap();
	println!("--> {}: {}ms", s, duration.as_secs_f64() / 1000.);
}


fn main() {
	time("Day 01", day01::day01);
	time("Day 06", day06::day06);
}
