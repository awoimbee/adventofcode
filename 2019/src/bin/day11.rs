#![feature(const_fn)]

extern crate int_vm;

use int_vm::{InputMode, OutputMode, Vm, VmState};
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
	Up,
	Right,
	Left,
	Down,
}

struct Robot {
	pos: (usize, usize),
	dir: Dir,
}

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
	let mut pannels = vec![vec![b' '; PANELS]; PANELS];
	let mut robot = Robot {pos: (PANELS/2, PANELS/2), dir: Dir::Up};
	let mut vm = Vm::new(code.clone(), vec![], InputMode::VecInterupt, OutputMode::No);
	while vm.state != VmState::Off {

		let curr_pan = &mut pannels[robot.pos.1][robot.pos.0];
		vm.input.push_back(match *curr_pan {b'#' => 1, _ => 0});
		let out = vm.run();
		*curr_pan = match out[0] {1 => b'#', _ => b'.'};

		robot.dir = match out[1] {
			0 if robot.dir == Dir::Down => Dir::Right,
			0 if robot.dir == Dir::Right => Dir::Up,
			0 if robot.dir == Dir::Up => Dir::Left,
			0 if robot.dir == Dir::Left => Dir::Down,
			1 if robot.dir == Dir::Down => Dir::Left,
			1 if robot.dir == Dir::Right => Dir::Down,
			1 if robot.dir == Dir::Up => Dir::Right,
			1 if robot.dir == Dir::Left => Dir::Up,
			_ => panic!("SALFHHHJLDHUVDSO"),
		};
		match robot.dir {
			Dir::Up => robot.pos.1 -= 1,
			Dir::Down => robot.pos.1 += 1,
			Dir::Left => robot.pos.0 -= 1,
			Dir::Right => robot.pos.0 += 1,
		};
		vm.output.clear();
    }

	let nb_pannels: usize = pannels.iter().map(|p| -> usize {p.iter().map(|p| if *p != b' ' {1} else {0}).sum()}).sum();
	unsafe { pannels.iter().map(|p| println!("{}", core::str::from_utf8_unchecked(p))).for_each(|_| ()) };
	println!("Nb pannels: {}", nb_pannels);


	/* PART II */
	let mut pannels = vec![vec![b' '; PANELS]; PANELS];
	let mut robot = Robot {pos: (PANELS/2, PANELS/2), dir: Dir::Up};
	pannels[robot.pos.1][robot.pos.0] = b'#';
	let mut vm = Vm::new(code, vec![], InputMode::VecInterupt, OutputMode::No);
	while vm.state != VmState::Off {
		let curr_pan = &mut pannels[robot.pos.1][robot.pos.0];
		vm.input.push_back(match *curr_pan {b'#' => 1, _ => 0});
		let out = vm.run();
		*curr_pan = match out[0] {1 => b'#', _ => b'.'};

		robot.dir = match out[1] {
			0 if robot.dir == Dir::Down => Dir::Right,
			0 if robot.dir == Dir::Right => Dir::Up,
			0 if robot.dir == Dir::Up => Dir::Left,
			0 if robot.dir == Dir::Left => Dir::Down,
			1 if robot.dir == Dir::Down => Dir::Left,
			1 if robot.dir == Dir::Right => Dir::Down,
			1 if robot.dir == Dir::Up => Dir::Right,
			1 if robot.dir == Dir::Left => Dir::Up,
			_ => panic!("SALFHHHJLDHUVDSO"),
		};
		match robot.dir {
			Dir::Up => robot.pos.1 -= 1,
			Dir::Down => robot.pos.1 += 1,
			Dir::Left => robot.pos.0 -= 1,
			Dir::Right => robot.pos.0 += 1,
		};
		vm.output.clear();
	}

	unsafe { pannels.iter().map(|p| println!("{}", core::str::from_utf8_unchecked(p))).for_each(|_| ()) };

	Ok(())
}
