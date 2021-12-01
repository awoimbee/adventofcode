use heapless::Vec;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day01.txt");
const INPUT_LINES: usize = 2000;

fn parse(input: &'static str) -> Vec<u32, INPUT_LINES> {
    input.split_terminator('\n').map(|s| s.parse().unwrap()).collect()
}

fn part_1(input: &[u32]) -> String {
    input
        .windows(2)
        .fold(0, |acc: u32, x: &[u32]| acc + (x[0] < x[1]) as u32)
        .to_string()
}

fn part_2(input: &[u32]) -> String {
    input
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .tuple_windows::<(_, _)>()
        .fold(0, |acc: u32, x| acc + (x.0 < x.1) as u32)
        .to_string()
}

pub fn day01() -> (String, String) {
    let input = parse(INPUT);

    let p1 = part_1(&input);
    let p2 = part_2(&input);
    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(part_1(&input), "7".to_string());
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(part_2(&input), "5".to_string());
    }
}
