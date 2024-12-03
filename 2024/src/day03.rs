use nom::Parser;

const INPUT: &str = include_str!("../input/day03.txt");

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul((u32, u32)),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let parse_instr_mul = nom::combinator::map(
        nom::sequence::delimited(
            nom::bytes::complete::tag("mul("),
            nom::sequence::separated_pair(
                nom::combinator::verify(
                    nom::character::complete::u32::<_, nom::error::Error<_>>,
                    |num: &u32| *num < 1000,
                ),
                nom::character::complete::char(','),
                nom::combinator::verify(
                    nom::character::complete::u32::<_, nom::error::Error<_>>,
                    |num: &u32| *num < 1000,
                ),
            ),
            nom::character::complete::char(')'),
        ),
        |mul| Some(Instruction::Mul(mul)),
    );
    let parse_instr_do =
        nom::combinator::map(nom::bytes::complete::tag("do()"), |_| Some(Instruction::Do));
    let parse_instr_dont = nom::combinator::map(nom::bytes::complete::tag("don't()"), |_| {
        Some(Instruction::Dont)
    });
    let parse_instruction = nom::branch::alt((parse_instr_do, parse_instr_dont, parse_instr_mul));

    let mut parser = nom::multi::many1(nom::branch::alt((
        parse_instruction,
        // This returns a lot of `None` :/
        nom::bytes::complete::take(1usize).map(|_| None),
    )));

    let (remaining, parsed) = parser.parse(input).expect("Could not parse input");
    assert_eq!(remaining.trim(), "");
    parsed.into_iter().flatten().collect()
}

fn solve(input: &str) -> (u32, u32) {
    let table = parse_input(input);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut dont = false;
    for instr in table {
        match instr {
            Instruction::Do => {
                dont = false;
            }
            Instruction::Dont => dont = true,
            Instruction::Mul((x, y)) => {
                part1 += x * y;
                if !dont {
                    part2 += x * y;
                }
            }
        }
    }

    (part1, part2)
}

pub fn day03() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_P1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_P2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT_P1).0, 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT_P2).1, 48);
    }
}
