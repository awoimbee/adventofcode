use anyhow::{anyhow, Result};
use fnv::FnvHashMap;
use nom::{
    bytes::complete::tag,
    character::complete,
    error::Error,
    sequence::{terminated, tuple},
};
use std::hint::unreachable_unchecked;

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
                        panic!();
                    }
                    *a %= b;
                }
                Instruction::Div((a, b)) => {
                    let b = self.values[*b as usize];
                    let a = &mut self.values[*a as usize];
                    if b == 0 {
                        panic!();
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
        let model_number = model_number.to_string();
        if model_number.contains('0') {
            return false;
        }
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

/// returns the value of z
fn day24_input_step(input: u64, mut z: i64, x_add: i64, y_add: u64, z_div: u64) -> i64 {
    if (z_div != 1 && z_div != 26) || input < 1 || input > 9 {
        unsafe {
            unreachable_unchecked();
        }
    }
    let eq = (z % 26 + x_add) as u64 == input;
    z /= z_div as i64; // either do nothing or pop the last element (/26)
    if !eq {
        z *= 26;
        z += (input + y_add) as i64;
    }
    z
}


fn day24_input_solver() -> (u64, u64) {
    let x_add = [14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9];
    let y_add = [8, 11, 2, 11, 1, 5, 10, 6, 1, 11, 9, 14, 11, 2];
    let z_div = [1, 1, 1, 26, 1, 26, 26, 1, 1, 1, 26, 26, 26, 26];


    let mut z_backlog = FnvHashMap::default();

    z_backlog.insert(0, [box [0u8; 14], box [0u8; 14]]);
    for idx in 0..14 {
        let mut new_z_backlog = FnvHashMap::<i64, [Box<[u8; 14]>; 2]>::default();
        for (z, mut paths) in z_backlog.into_iter() {
            for input in 1..10u8 {
                let new_z = day24_input_step(input as u64, z, x_add[idx], y_add[idx], z_div[idx]);
                if !(0..1_000_000).contains(&new_z) {
                    continue;
                }
                paths[0][idx] = input;
                paths[1][idx] = input;
                if let Some(other_paths) = new_z_backlog.get_mut(&new_z) {
                    if other_paths[0][idx] > paths[0][idx] {
                        other_paths[0] = paths[0].clone();
                    }
                    if other_paths[1][idx] < paths[1][idx] {
                        other_paths[1] = paths[1].clone();
                    }
                } else {
                    new_z_backlog.insert(new_z, paths.clone());
                }
            }
        }
        z_backlog = new_z_backlog;
    }
    z_backlog
        .into_iter()
        .filter(|(z, _)| *z == 0)
        .flat_map(|(_, paths)| {
            paths.into_iter().map(
                |p|
                p.into_iter().map(|comp| (comp + b'0') as char)
                .collect::<String>()
                .parse::<u64>()
                .unwrap())
        })
        .fold((u64::MAX, u64::MIN), |mut acc, elem| {
            acc.0 = acc.0.min(elem);
            acc.1 = acc.1.max(elem);
            acc
        })
}


pub fn day24() -> (String, String) {
    let mut monad = Input::from_str(INPUT).unwrap();
    let (part2, part1) = day24_input_solver();
    debug_assert!({
        monad.reset();
        monad.run(part1)
    });
    debug_assert!({
        monad.reset();
        monad.run(part2)
    });

    (part1.to_string(), part2.to_string())
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

    #[test]
    fn test_part1_reversed_engineered_program() {
        let mut input = Input::from_str(INPUT).unwrap();
        let test_values = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 2, 3, 7, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6],
        ];
        for test_val in test_values.iter() {
            let reg_z = part1_reversed_engineered_program(test_val);
            input._run_vm(test_val.iter().rev().map(|&v| v as u8).collect());
            assert_eq!(input.values[2], reg_z, "input: {:?}", test_val);
            input.reset();
        }
    }
}
