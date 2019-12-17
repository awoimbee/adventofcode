#![feature(const_fn)]

extern crate int_vm;

use int_vm::{InputMode, OutputMode, Vm};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let now = std::time::Instant::now();
    let mut vm = Vm::new(code.clone(), vec![1], InputMode::VecDirect, OutputMode::No);
    println!("Part. I: {} (took {}µs)", vm.run()[0], now.elapsed().as_micros());

    let mut vm = Vm::new(code, vec![2], InputMode::VecDirect, OutputMode::No);
    println!("Part. II: {} (took {}µs)", vm.run()[0], now.elapsed().as_micros());
}
