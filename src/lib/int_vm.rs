#![feature(const_fn)]

use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

const MEM_SIZE: usize = 6_000;

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

#[cfg(test)]
mod tests {
    use super::*;

    const DAY9: [i64; 973] = [1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,344,1023,1101,0,0,1020,1101,0,481,1024,1102,1,1,1021,1101,0,24,1005,1101,0,29,1018,1102,39,1,1019,1102,313,1,1028,1102,1,35,1009,1101,28,0,1001,1101,26,0,1013,1101,0,351,1022,1101,564,0,1027,1102,1,32,1011,1101,23,0,1006,1102,1,25,1015,1101,21,0,1003,1101,0,31,1014,1101,33,0,1004,1102,37,1,1000,1102,476,1,1025,1101,22,0,1007,1102,30,1,1012,1102,1,27,1017,1102,1,34,1002,1101,38,0,1008,1102,1,36,1010,1102,1,20,1016,1102,567,1,1026,1102,1,304,1029,109,-6,2108,35,8,63,1005,63,201,1001,64,1,64,1106,0,203,4,187,1002,64,2,64,109,28,21101,40,0,-9,1008,1013,38,63,1005,63,227,1001,64,1,64,1105,1,229,4,209,1002,64,2,64,109,-2,1205,1,243,4,235,1105,1,247,1001,64,1,64,1002,64,2,64,109,-12,2102,1,-5,63,1008,63,24,63,1005,63,271,1001,64,1,64,1105,1,273,4,253,1002,64,2,64,109,8,2108,22,-9,63,1005,63,295,4,279,1001,64,1,64,1106,0,295,1002,64,2,64,109,17,2106,0,-5,4,301,1001,64,1,64,1106,0,313,1002,64,2,64,109,-21,21107,41,40,7,1005,1019,333,1001,64,1,64,1105,1,335,4,319,1002,64,2,64,109,1,2105,1,10,1001,64,1,64,1105,1,353,4,341,1002,64,2,64,109,10,1206,-3,371,4,359,1001,64,1,64,1105,1,371,1002,64,2,64,109,-5,21108,42,42,-7,1005,1011,393,4,377,1001,64,1,64,1105,1,393,1002,64,2,64,109,-8,2101,0,-4,63,1008,63,23,63,1005,63,415,4,399,1105,1,419,1001,64,1,64,1002,64,2,64,109,13,21102,43,1,-6,1008,1017,43,63,1005,63,441,4,425,1106,0,445,1001,64,1,64,1002,64,2,64,109,-21,1207,0,33,63,1005,63,465,1001,64,1,64,1106,0,467,4,451,1002,64,2,64,109,19,2105,1,3,4,473,1106,0,485,1001,64,1,64,1002,64,2,64,109,1,21101,44,0,-7,1008,1015,44,63,1005,63,511,4,491,1001,64,1,64,1106,0,511,1002,64,2,64,109,2,1206,-3,527,1001,64,1,64,1105,1,529,4,517,1002,64,2,64,109,-8,1201,-7,0,63,1008,63,35,63,1005,63,555,4,535,1001,64,1,64,1105,1,555,1002,64,2,64,109,1,2106,0,10,1105,1,573,4,561,1001,64,1,64,1002,64,2,64,109,4,21107,45,46,-7,1005,1014,591,4,579,1106,0,595,1001,64,1,64,1002,64,2,64,109,-12,1208,-6,21,63,1005,63,617,4,601,1001,64,1,64,1105,1,617,1002,64,2,64,109,-11,1208,6,31,63,1005,63,637,1001,64,1,64,1106,0,639,4,623,1002,64,2,64,109,16,2101,0,-7,63,1008,63,20,63,1005,63,659,1105,1,665,4,645,1001,64,1,64,1002,64,2,64,109,3,2102,1,-9,63,1008,63,38,63,1005,63,691,4,671,1001,64,1,64,1106,0,691,1002,64,2,64,109,4,1205,-1,703,1105,1,709,4,697,1001,64,1,64,1002,64,2,64,109,-14,21108,46,45,7,1005,1014,729,1001,64,1,64,1105,1,731,4,715,1002,64,2,64,109,7,21102,47,1,0,1008,1014,45,63,1005,63,755,1001,64,1,64,1106,0,757,4,737,1002,64,2,64,109,-12,2107,34,7,63,1005,63,775,4,763,1105,1,779,1001,64,1,64,1002,64,2,64,109,-5,1207,6,22,63,1005,63,797,4,785,1106,0,801,1001,64,1,64,1002,64,2,64,109,12,1202,0,1,63,1008,63,35,63,1005,63,827,4,807,1001,64,1,64,1105,1,827,1002,64,2,64,109,-5,1202,0,1,63,1008,63,36,63,1005,63,851,1001,64,1,64,1105,1,853,4,833,1002,64,2,64,109,-2,1201,4,0,63,1008,63,20,63,1005,63,873,1105,1,879,4,859,1001,64,1,64,1002,64,2,64,109,2,2107,22,-1,63,1005,63,899,1001,64,1,64,1106,0,901,4,885,4,64,99,21102,1,27,1,21101,0,915,0,1105,1,922,21201,1,53897,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1106,0,922,21202,1,1,-1,21201,-2,-3,1,21101,0,957,0,1105,1,922,22201,1,-1,-2,1105,1,968,22102,1,-2,-2,109,-3,2105,1,0];

    #[test]
    fn test_day09_p1() {
        let t: &[i64] = &DAY9;
        let mut vm = Vm::new(Vec::from(t), vec![1], InputMode::VecDirect, OutputMode::No);
        vm.run();
        assert_eq!(vm.output.len(), 1);
        assert_eq!(vm.output[0], 4080871669);
    }

    #[test]
    fn test_day09_p2() {
        let t: &[i64] = &DAY9;
        let mut vm = Vm::new(Vec::from(t), vec![2], InputMode::VecDirect, OutputMode::No);
        vm.run();
        assert_eq!(vm.output.len(), 1);
        assert_eq!(vm.output[0], 75202);
    }
}
