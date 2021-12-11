const INPUT: &str = include_str!("../input/day10.txt");

fn solve(input: &str) -> (u64, u64) {
    let mut score1 = 0;
    let mut score2 = Vec::new();
    for line in input.split_terminator('\n') {
        let mut expected_terminator = Vec::new();
        let mut line_score1 = 0;
        for c in line.as_bytes() {
            match c {
                b'{' => expected_terminator.push(b'}'),
                b'[' => expected_terminator.push(b']'),
                b'<' => expected_terminator.push(b'>'),
                b'(' => expected_terminator.push(b')'),
                b'}' | b']' | b'>' | b')' => {
                    if expected_terminator.pop() != Some(*c) {
                        line_score1 += match c {
                            b')' => 3,
                            b']' => 57,
                            b'>' => 25137,
                            b'}' => 1197,
                            _ => panic!(),
                        }
                    }
                }
                _ => panic!(),
            }
        }
        if line_score1 == 0 {
            let mut line_score2 = 0;
            for term in expected_terminator.into_iter().rev() {
                line_score2 *= 5;
                line_score2 += match term {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!(),
                };
            }
            if line_score2 > 0 {
                score2.push(line_score2);
            }
        }
        score1 += line_score1;
    }
    score2.sort_unstable();

    let score2 = score2[score2.len() / 2];
    (score1, score2)
}

pub fn day10() -> (String, String) {
    let (part1, part2) = solve(INPUT);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1_test_input() {
        assert_eq!(solve(TEST_INPUT).0, 26397);
    }

    #[test]
    fn test_part_2_test_input() {
        assert_eq!(solve(TEST_INPUT).1, 288957);
    }
}
