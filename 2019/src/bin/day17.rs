#![feature(const_fn)]

extern crate int_vm;

use int_vm::{InputMode, OutputMode, Vm, VmState};
use std::io::{self, BufRead};

// #[derive(Clone, Copy, Debug, PartialEq)]
// enum Dir {
// 	Up,
// 	Right,
// 	Left,
// 	Down,
// }

// struct Robot {
// 	pos: (usize, usize),
// 	dir: Dir,
// }

const PANELS: usize = 180;

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
	println!("Please input the incode program, followed by \\n");
	let code: Vec<i64> =
		stdin.lock().lines().next().unwrap()?
			.trim()
			.split(',')
			.map(|s| s.parse().unwrap())
			.collect();

	/* PART I */

	// let mut pannels = vec![vec![b' '; PANELS]; PANELS];
	// let mut robot = Robot {pos: (PANELS/2, PANELS/2), dir: Dir::Up};
	let mut vm = Vm::new(code.clone(), vec![], InputMode::VecDirect, OutputMode::No);
	vm.run();
	let map = vm.output;
	let s: String = map.iter().map(|&x| x as u8 as char).collect();
	println!("{}", s);
	let map_width = map.iter().position(|&x| x == b'\n' as i64).unwrap();
	let map_height = map.len() / map_width;
	let mut alignment_param_sum = 0;
	for i in map_width..map.len()-map_width {
		const HASH: i64 = b'#' as i64;

		if map[i] == HASH && map[i+1] == HASH && map[i-1] == HASH && map[i-map_width] == HASH && map[i+map_height] == HASH {
			let alignment_param = (i / map_width) + (i % map_width);
			alignment_param_sum += alignment_param;
		}
	}
	println!("Part. I: {}", alignment_param_sum);

	Ok(())
}
