#![feature(const_fn)]

use std::io::{self, BufRead};

struct Instruction {
    exec: &'static dyn Fn(&mut Machine),
    nb_params: usize,
    arg_store: usize,
    name: &'static str,
}
impl Instruction {
    pub const fn new(
        exec: &'static dyn Fn(&mut Machine),
        nb_params: usize,
        arg_store: usize,
        name: &'static str,
    ) -> Instruction {
        Instruction {
            exec,
            nb_params,
            arg_store,
            name,
        }
    }
}

const INSTRUCTIONS: [Instruction; 9] = [
    Instruction::new(&Machine::null, 0, 99, "Should never happen"),
    Instruction::new(&Machine::add, 3, 2, "add"),
    Instruction::new(&Machine::mul, 3, 2, "mul"),
    Instruction::new(&Machine::inp, 1, 0, "input"),
    Instruction::new(&Machine::out, 1, 99, "output"),
    Instruction::new(&Machine::jmp_true, 2, 99, "jump-if-true"),
    Instruction::new(&Machine::jmp_false, 2, 99, "jump-if-false"),
    Instruction::new(&Machine::cmp_le, 3, 2, "less than"),
    Instruction::new(&Machine::cmp_eq, 3, 2, "equals"),
];

struct Machine {
    pub pc: usize, // programm counter / instruction pointer
    pub reg: [i32; 4],
    pub ram: Vec<i32>,
}
impl Machine {
    pub fn new(ram: Vec<i32>) -> Machine {
        Machine {
            pc: 0,
            reg: [0, 0, 0, 0],
            ram,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let tab: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut machine = Machine::new(tab);
    machine.run();
}

impl Machine {
    pub fn run(&mut self) {
        loop {
            self.run_one()
        }
    }
    pub fn run_one(&mut self) {
        let rawcode = self.ram[self.pc] as usize;
        let opcode = rawcode % 100;
        if opcode >= INSTRUCTIONS.len() || opcode == 0 {
            match opcode == 99 {
                true => {
                    eprintln!("{:?}", self.ram);
                    std::process::exit(0);
                }
                false => panic!("ERROR: opcode is {}\n{:?}", opcode, self.ram),
            }
        }
        let inst = &INSTRUCTIONS[opcode];
        const FETCH_MODES: [usize; 5] = [100, 1000, 10_000, 100_000, 1_000_000];
        for i in 0..inst.nb_params as usize {
            if i == inst.arg_store {
                self.reg[i] = self.ram[self.pc + 1 + i];
                continue;
            }
            let fetch_mode = rawcode % FETCH_MODES[i + 1] / FETCH_MODES[i];
            let arg = self.ram[self.pc + 1 + i];
            match fetch_mode {
                1 => self.reg[i] = arg,                    // immediate
                0 => self.reg[i] = self.ram[arg as usize], // direct
                _ => panic!("Fetch_mode is fucked"),
            };
        }
        eprintln!(
            "{} reg: {:?}",
            inst.name,
            &self.reg[..inst.nb_params as usize]
        );
        self.pc += 1 + inst.nb_params as usize;
        (inst.exec)(self);
        eprintln!("pc move to {}", self.pc);
    }
}

// Instructions
impl Machine {
    pub fn null(&mut self) {}
    // pub fn exit(&mut self) {
    // 	std::process::exit(0);
    // }
    pub fn add(&mut self) {
        let dst = &mut self.ram[self.reg[2] as usize];
        *dst = self.reg[0] + self.reg[1];
    }
    pub fn mul(&mut self) {
        let dst = &mut self.ram[self.reg[2] as usize];
        *dst = self.reg[0] * self.reg[1];
    }
    pub fn inp(&mut self) {
        let input = io::stdin().lock().lines().next().unwrap().unwrap();
        let dst = &mut self.ram[self.reg[0] as usize];
        *dst = input.parse().unwrap();
    }
    pub fn out(&mut self) {
        println!("OUT: {}", self.reg[0]);
    }
    pub fn jmp_true(&mut self) {
        if self.reg[0] != 0 {
            self.pc = self.reg[1] as usize;
        }
    }
    pub fn jmp_false(&mut self) {
        if self.reg[0] == 0 {
            self.pc = self.reg[1] as usize;
        }
    }
    pub fn cmp_le(&mut self) {
        self.ram[self.reg[2] as usize] = (self.reg[0] < self.reg[1]) as i32;
    }
    pub fn cmp_eq(&mut self) {
        self.ram[self.reg[2] as usize] = (self.reg[0] == self.reg[1]) as i32;
    }
}
