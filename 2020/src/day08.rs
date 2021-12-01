const INPUT: &str = include_str!("../input/day08.txt");

#[derive(Clone, Copy)]
enum Inst {
    Nop,
    Acc,
    Jmp,
}

struct Instruction {
    pub op: Inst,
    pub nb: i32,
}

impl Instruction {
    pub fn new(op: Inst, nb: i32) -> Self {
        Self { op, nb }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut t = s.split(' ');
        let inst_str = t.next().unwrap();
        let nb = t.next().unwrap().parse().unwrap();
        match inst_str {
            "nop" => Self::new(Inst::Nop, nb),
            "acc" => Self::new(Inst::Acc, nb),
            "jmp" => Self::new(Inst::Jmp, nb),
            _ => unreachable!(),
        }
    }
}

struct VM {
    pub accumulator: i32,
}

impl VM {
    pub fn default() -> Self {
        Self { accumulator: 0 }
    }

    pub fn run(&mut self, code: &[Instruction]) -> bool {
        let mut visited = vec![false; code.len()];
        let mut pc = 0;
        while pc < code.len() {
            if visited[pc] {
                return false;
            }
            let wrapped_instr = &code[pc];
            visited[pc] = true;
            match wrapped_instr.op {
                Inst::Nop => (),
                Inst::Acc => self.accumulator += wrapped_instr.nb,
                Inst::Jmp => {
                    pc = (pc as i64 + wrapped_instr.nb as i64) as usize;
                    continue;
                }
            };
            pc += 1;
        }
        true
    }
}

fn parse() -> Vec<Instruction> {
    let mut instructions = Vec::new();
    INPUT
        .lines()
        .for_each(|l| instructions.push(Instruction::from(l)));
    instructions
}

fn part_1(code: &[Instruction]) -> i32 {
    let mut machine = VM::default();
    machine.run(code);
    machine.accumulator
}

fn part_2_invert(instr: &mut Instruction) {
    if let Inst::Jmp = instr.op {
        instr.op = Inst::Nop;
    } else {
        instr.op = Inst::Jmp;
    }
}

fn part_2(code: &mut [Instruction]) -> i32 {
    let mut i = 0;
    loop {
        let instr = unsafe { code.get_unchecked_mut(i) };
        if matches!(instr.op, Inst::Jmp | Inst::Nop) {
            part_2_invert(instr);
            let mut machine = VM::default();
            if machine.run(&code) {
                return machine.accumulator;
            }
            let instr = unsafe { code.get_unchecked_mut(i) };
            part_2_invert(instr);
        }
        i += 1;
    }
}

pub fn day08() -> (String, String) {
    let mut instructions = parse();

    (
        format!("{}", part_1(&instructions)),
        format!("{}", part_2(&mut instructions)),
    )
}
