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

    let mut vm = Vm::new(code, vec![], InputMode::Stdin, OutputMode::Stderr);
    vm.run();
}
