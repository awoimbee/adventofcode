#![feature(const_fn)]

extern crate int_machine;
// use lib::int_machine;

use std::io::{self, BufRead};
use int_machine::{Machine, OutputMode, InputMode};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let code: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
		.collect();
	let mut max_output = 0;
	let mut max_params = [0, 0, 0, 0, 0];

	for a in 0..=4 {
		let mut ma = Machine::new(code.clone(), vec![a, 0], InputMode::VecDirect, OutputMode::No);
		ma.run();
		for b in 0..=4 {
			if b == a {
				continue;
			}
			let mut mb = Machine::new(code.clone(), vec![b, ma.output[0]], InputMode::VecDirect, OutputMode::No);
			mb.run();
			for c in 0..=4 {
				if c == a || c == b {
					continue;
				}
				let mut mc = Machine::new(code.clone(), vec![c, mb.output[0]], InputMode::VecDirect, OutputMode::No);
				mc.run();
				for d in 0..=4 {
					if d == a || d == b || d == c {
						continue;
					}
					let mut md = Machine::new(code.clone(), vec![d, mc.output[0]], InputMode::VecDirect, OutputMode::No);
					md.run();
					for e in 0..=4 {
						if e == a || e == b || e == c || e == d {
							continue;
                        }
						let mut me = Machine::new(code.clone(), vec![e, md.output[0]], InputMode::VecDirect, OutputMode::No);
						me.run();
						let re = me.output[0];
						if re > max_output {
							max_output = re;
							max_params = [a, b, c, d, e];
						}

					}
				}
			}
		}
	}
	println!("Max output: {}", max_output);
	println!("Params: {:?}", max_params);
}
