#![feature(const_fn)]

use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

const MEM_SIZE: usize = 2_000;

macro_rules! log {
    ($($arg:tt)*) => (if cfg! (debug_assertions) { println!($($arg)*) } )
}

#[derive(PartialEq)]
pub enum OutputMode {
    Stdout,
    Stderr,
    No,
}

#[derive(PartialEq)]
pub enum InputMode {
    Stdin,
    VecDirect,
    VecInterupt,
}

#[derive(PartialEq, Debug)]
pub enum VmState {
    On,
    Halt,
    Off,
}

pub struct Vm {
    pc: usize,
    rel_base: i64,
    reg: [i64; 4],
    ram: Vec<i64>,
    pub state: VmState,
    pub input: VecDeque<i64>,
    i: InputMode,
    o: OutputMode,
    pub output: Vec<i64>,
}

impl Vm {
    pub fn new(mut ram: Vec<i64>, input: Vec<i64>, i: InputMode, o: OutputMode) -> Self {
        let input = VecDeque::from(input);
        if ram.len() < MEM_SIZE {
            ram.extend((0..MEM_SIZE - ram.len()).map(|_| 0))
        };
        Vm {
            pc: 0,
            rel_base: 0,
            reg: [0, 0, 0, 0],
            ram,
            state: VmState::Halt,
            input,
            i,
            o,
            output: Vec::new(),
        }
    }
}

struct Instruction {
    exec: &'static dyn Fn(&mut Vm),
    nb_params: u8,
    st: u8,
    name: &'static str,
}
impl Instruction {
    pub const fn new(
        exec: &'static dyn Fn(&mut Vm),
        nb_params: u8,
        st: u8,
        name: &'static str,
    ) -> Instruction {
        Instruction {
            exec,
            nb_params,
            st,
            name,
        }
    }
}
#[rustfmt::skip]
const INSTRUCTIONS: [Instruction; 10] = [
    Instruction::new(&Vm::exit,        0, 99, "exit"),
    Instruction::new(&Vm::add,         3,  2, "add"),
    Instruction::new(&Vm::mul,         3,  2, "mul"),
    Instruction::new(&Vm::inp,         1,  0, "input"),
    Instruction::new(&Vm::out,         1, 99, "output"),
    Instruction::new(&Vm::jmp_true,    2, 99, "jump-if-true"),
    Instruction::new(&Vm::jmp_false,   2, 99, "jump-if-false"),
    Instruction::new(&Vm::cmp_le,      3,  2, "less than"),
    Instruction::new(&Vm::cmp_eq,      3,  2, "equals"),
    Instruction::new(&Vm::st_rel_base, 1, 99, "set-rel-base"),
];
// Instructions
impl Vm {
    #[rustfmt::skip]
    pub fn exit(&mut self) {
        self.pc -= 1;
        let opcode = self.ram[self.pc];
        if opcode != 99 { eprintln!("ERROR: unknown opcode {}", opcode) };
        log!("{:?}", self.ram);
        self.state = VmState::Off;
    }
    pub fn add(&mut self) {
        let dst = &mut self.ram[self.reg[2] as usize];
        *dst = self.reg[0] + self.reg[1];
    }
    pub fn mul(&mut self) {
        let dst = &mut self.ram[self.reg[2] as usize];
        *dst = self.reg[0] * self.reg[1];
    }
    pub fn inp(&mut self) {
        let dst = &mut self.ram[self.reg[0] as usize];
        if self.i == InputMode::Stdin {
            print!("Input a number: ");
            io::stdout().flush().unwrap();
            let input = io::stdin().lock().lines().next().unwrap().unwrap();
            *dst = input.parse().unwrap();
        } else {
            let input = match self.input.pop_front() {
                Some(i) => i,
                None if self.i == InputMode::VecInterupt => {
                    log!("No input, halting");
                    self.pc -= 2;
                    self.state = VmState::Halt;
                    return;
                }
                None => panic!("No input left"),
            };
            *dst = input;
        }
    }
    pub fn out(&mut self) {
        self.output.push(self.reg[0]);
        match self.o {
            OutputMode::Stderr => eprintln!("{}", self.reg[0]),
            OutputMode::Stdout => println!("{}", self.reg[0]),
            _ => (),
        };
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
        self.ram[self.reg[2] as usize] = (self.reg[0] < self.reg[1]).into();
    }
    pub fn cmp_eq(&mut self) {
        self.ram[self.reg[2] as usize] = (self.reg[0] == self.reg[1]).into();
    }
    pub fn st_rel_base(&mut self) {
        self.rel_base += self.reg[0];
    }
}

impl Vm {
    pub fn run(&mut self) -> &Vec<i64> {
        log!("RUN");
        self.state = VmState::On;
        while self.state == VmState::On {
            self.run_one()
        }
        log!("STOP: {:?}", self.state);
        &self.output
    }
    #[rustfmt::skip]
    pub fn run_one(&mut self) {
        let rawcode = self.ram[self.pc] as usize;
        let mut opcode = rawcode % 100;
        if opcode >= INSTRUCTIONS.len() { opcode = 0 };
        let inst = &INSTRUCTIONS[opcode];
        const FETCH_MODES: [usize; 5] = [100, 1000, 10_000, 100_000, 1_000_000];
        for i in 0..inst.nb_params as usize {
            let fetch_mode = rawcode % FETCH_MODES[i + 1] / FETCH_MODES[i];
            let arg = self.ram[self.pc + 1 + i];
            self.reg[i] = match fetch_mode {
                2 if i != inst.st.into() => self.ram[(self.rel_base + arg) as usize], // relative (ld)
                2 if i == inst.st.into() => self.rel_base + arg,                      // relative (st)
                1 if i != inst.st.into() => arg,                                      // immediate (ld)
                0 if i != inst.st.into() => self.ram[arg as usize],                   // position (ld)
                0 if i == inst.st.into() => arg,                                      // position (st)
                _ => panic!("Fetch_mode is fucked"),
            };
        }
        log!("{:10} reg: {:?}", inst.name, &self.reg[..inst.nb_params as usize]);
        self.pc += 1 + inst.nb_params as usize;
        (inst.exec)(self);
        log!("-> pc move to {}", self.pc);
    }
}
