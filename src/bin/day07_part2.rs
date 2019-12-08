#![feature(const_fn)]

extern crate int_machine;

use int_machine::{InputMode, Machine, MachineState, OutputMode};
use std::collections::HashSet;
use std::io::{self, BufRead};

fn run_vm_arg(vm: &mut Machine, arg: i32) -> i32 {
    vm.input.push_back(arg);
    vm.run();
    vm.output.pop().unwrap()
}

fn feedback_loop(prog: &Vec<i32>, phase_seq: [i32; 5]) -> i32 {
    let new = |ps| {
        Machine::new(
            prog.clone(),
            vec![ps],
            InputMode::VecInterupt,
            OutputMode::No,
        )
    };
    let mut ma = new(phase_seq[0]);
    let mut mb = new(phase_seq[1]);
    let mut mc = new(phase_seq[2]);
    let mut md = new(phase_seq[3]);
    let mut me = new(phase_seq[4]);
    me.state = MachineState::Halt;
    let mut out = 0;
    while me.state != MachineState::Off {
        out = run_vm_arg(&mut ma, out);
        out = run_vm_arg(&mut mb, out);
        out = run_vm_arg(&mut mc, out);
        out = run_vm_arg(&mut md, out);
        out = run_vm_arg(&mut me, out);
    }
    println!("{:?} => {}", phase_seq, out);
    out
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let code: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut max_output = 0;

    for i in 5..10 {
        for j in 5..10 {
            for k in 5..10 {
                for l in 5..10 {
                    for m in 5..10 {
                        let phase_seq = [i, j, k, l, m];
                        let ps_set: HashSet<i32> = vec![i, j, k, l, m].into_iter().collect();
                        if ps_set.len() == 5 {
                            let result = feedback_loop(&code, phase_seq);
                            if result > max_output {
                                max_output = result
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Max output: {}", max_output);
}
