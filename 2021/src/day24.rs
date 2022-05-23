use std::hint::unreachable_unchecked;
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete,
    error::Error,
    sequence::{terminated, tuple},
};

const INPUT: &str = include_str!("../input/day24.txt");

struct Input {
    values: Vec<i64>,
    instructions: Vec<Instruction>,
}

/// Stores indexes to values in Input.values
enum Instruction {
    Inp(u8),
    Add((u8, u8)),
    Mul((u8, u8)),
    Mod((u8, u8)),
    Div((u8, u8)),
    Eql((u8, u8)),
}

impl Input {
    fn from_str(s: &'static str) -> Result<Self> {
        // x, y, z, w are 0..4
        let mut values = vec![0; 4];
        let mut instructions = vec![];
        for l in s.lines() {
            let (s, instruction) =
                terminated::<_, _, _, Error<&str>, _, _>(complete::alpha0, tag(" "))(l)?;
            let instr = if instruction == "inp" {
                match s {
                    "x" => Instruction::Inp(0),
                    "y" => Instruction::Inp(1),
                    "z" => Instruction::Inp(2),
                    "w" => Instruction::Inp(3),
                    _ => return Err(anyhow!("Invalid input register `{}`", s)),
                }
            } else {
                let (b, (a, _)) = tuple::<_, _, Error<&str>, _>((complete::alpha1, tag(" ")))(s)?;
                let a = match a {
                    "x" => 0,
                    "y" => 1,
                    "z" => 2,
                    "w" => 3,
                    _ => {
                        return Err(anyhow!(
                            "The first arg to an instruction must be a variable"
                        ))
                    }
                };
                let b = match b {
                    "x" => 0,
                    "y" => 1,
                    "z" => 2,
                    "w" => 3,
                    _ => {
                        values.push(b.parse::<i64>()?);
                        (values.len() - 1) as u8
                    }
                };
                match instruction {
                    "add" => Instruction::Add((a, b)),
                    "mul" => Instruction::Mul((a, b)),
                    "mod" => Instruction::Mod((a, b)),
                    "div" => Instruction::Div((a, b)),
                    "eql" => Instruction::Eql((a, b)),
                    _ => return Err(anyhow!("Invalid instruction `{}`", instruction)),
                }
            };
            instructions.push(instr);
        }
        Ok(Input {
            values,
            instructions,
        })
    }

    /// The first input is at the end of the Vec<_>
    fn _run_vm(&mut self, mut input: Vec<u8>) -> bool {
        for instr in self.instructions.iter() {
            match instr {
                Instruction::Inp(i) => {
                    let val = input.pop().expect("No more digit for inp to read !");
                    self.values[*i as usize] = val as i64;
                }
                Instruction::Add((a, b)) => {
                    self.values[*a as usize] += self.values[*b as usize];
                }
                Instruction::Mul((a, b)) => {
                    self.values[*a as usize] *= self.values[*b as usize];
                }
                Instruction::Mod((a, b)) => {
                    let b = self.values[*b as usize];
                    let a = &mut self.values[*a as usize];
                    if *a < 0 || b <= 0 {
                        return false;
                    }
                    *a %= b;
                }
                Instruction::Div((a, b)) => {
                    let b = self.values[*b as usize];
                    let a = &mut self.values[*a as usize];
                    if b == 0 {
                        return false;
                    }
                    *a /= b;
                }
                Instruction::Eql((a, b)) => {
                    let val = (self.values[*a as usize] == self.values[*b as usize]) as i64;
                    self.values[*a as usize] = val;
                }
            }
        }
        self.values[2] == 0
    }

    fn run(&mut self, model_number: u64) -> bool {
        // debug_assert_eq!(model_number & 0b1, 0);
        // debug_assert_eq!(model_number & (0b11 << 14), 0);
        let model_number = model_number.to_string();
        if model_number.contains('0') {
            return false;
        }
        println!("{}", model_number);
        let model_nb_inp = model_number
            .into_bytes()
            .into_iter()
            .rev()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        self._run_vm(model_nb_inp)
    }

    /// reset the registers
    fn reset(&mut self) {
        self.values[0] = 0;
        self.values[1] = 0;
        self.values[2] = 0;
        self.values[3] = 0;
    }
}

const fn part1_reversed_engineered_program(input: &[i64; 14]) -> bool {
    // ^add (.+) (.+)$ => $1 += $2;
    // ^inp (.+)$ => $1 = input.pop().unwrap();
    // ^eql (.+) (.+)$ => $1 = if $1 == $2 {1} else {0};
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;

    w = input[10];
    if w < 1 || w > 9 {
        unsafe{unreachable_unchecked();}
    }
    z = ((((input[0]+8 + input[1]+11)*26+(input[3]+11))*26+(input[6]+10))*26+(input[7]+6))*26+(input[8]+1);
    z *= if input[9]+5 == input[10] { 1 } else { 26 };
    z += if input[9]+5 == input[10] {0} else{input[10]+9};
    w = input[11];
    if w < 1 || w > 9 {
        unsafe{unreachable_unchecked();}
    }
    x = z%26-6;
    z /= 26;
    x = if x == w { 0 } else { 1 };
    y = 25*x+1;
    z *= y;
    y = 0;
    y += w;
    y += 14;
    y *= x;
    z += y;
    w = input[12];
    if w < 1 || w > 9 {
        unsafe{unreachable_unchecked();}
    }
    x = z%26-2;
    z /= 26;
    x = if x == w { 0 } else { 1 };
    y = 25*x+1;
    z *= y;
    y = 0;
    y += w;
    y += 11;
    y *= x;
    z += y;
    w = input[13];
    if w < 1 || w > 9 {
        unsafe{unreachable_unchecked();}
    }
    x = z%26-9;
    z /= 26;
    x = if x == w { 0 } else { 1 };
    y =  25*x+1;
    z *= y;
    y = w+2*x;
    z += y;

    z == 1
}

fn part1() -> [i64; 14] {
    let mut model_number = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,9,9];
    let mut i = 13;
    let mut j = 0;
    loop {
        model_number[i] -= 1;
        if model_number[i] == 0 {
            model_number[i] = 9;
            i -= 1;
            if i == 2 || i == 4 || i == 5 {
                i -= 1;
            }
            continue;
        } else {
            i = 13;
        }
        j += 1;
        if j % 10_000_000 == 0 {
            println!("model_nb: {:?}", model_number);
        }
        // model_number -= 1;
        // let m_n_str = model_number.to_string();

        // if m_n_str.contains('0') {
        //     continue;
        // }
        // if model_number % 111 == 0 {
        //     println!("{}", m_n_str);
        // }
        // let model_nb_inp = m_n_str
        //     .into_bytes()
        //     .into_iter()
        //     .rev()
        //     .map(|b| (b - b'0') as i64)
        //     .collect::<Vec<_>>();
        if part1_reversed_engineered_program(&model_number) {
            break;
        }

    }
    model_number
}

pub fn day24() -> (String, String) {
    let mut input = Input::from_str(INPUT).unwrap();
    let part1 = part1().iter().map(|&nb| (b'0' + nb as u8) as char).collect::<String>();
    let part2 = "".to_string();

    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negate() {
        let test_neg = "inp x\nmul x -1";
        let mut input = Input::from_str(test_neg).unwrap();
        for i in 0..9 {
            input._run_vm(vec![i as u8]);
            assert_eq!(input.values[0], -i);
            input.reset();
        }
    }

    #[test]
    fn test_3times_eq() {
        let test_3times_eq = "inp z\ninp x\nmul z 3\neql z x\n";
        let mut input = Input::from_str(test_3times_eq).unwrap();
        for i in 0..9 {
            for j in 0..9 {
                input._run_vm(vec![i as u8, j as u8]);
                assert_eq!(input.values[2], (i == j * 3) as i64, "i={}, j={}", i, j);
                input.reset();
            }
        }
    }

    #[test]
    fn test_to_binary() {
        let test_binary = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";
        let mut input = Input::from_str(test_binary).unwrap();
        for i in 0..9 {
            let binary = format!("{:010b}", i);
            let binary_vec = binary
                .into_bytes()
                .into_iter()
                .map(|b| (b - b'0') as i64)
                .collect::<Vec<_>>();
            input._run_vm(vec![i]);
            assert_eq!(input.values[2], binary_vec[9]);
            assert_eq!(input.values[1], binary_vec[8]);
            assert_eq!(input.values[0], binary_vec[7]);
            assert_eq!(input.values[3], binary_vec[6]);
            input.reset();
        }
    }
}
