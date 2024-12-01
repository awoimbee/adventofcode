const INPUT: &str = include_str!("../input/day01.txt");

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let line_parser = nom::sequence::terminated(
        nom::sequence::separated_pair(
            nom::character::complete::u32::<_, nom::error::Error<_>>,
            nom::character::complete::space1,
            nom::character::complete::u32,
        ),
        nom::character::complete::line_ending,
    );
    let mut parser = nom::multi::fold_many1(
        line_parser,
        || (Vec::new(), Vec::new()),
        |(mut vec_a, mut vec_b), (a, b)| {
            vec_a.push(a);
            vec_b.push(b);
            (vec_a, vec_b)
        },
    );
    let (remaining, parsed) = parser(input).expect("Could not parse input");
    assert!(remaining == "");
    parsed
}

fn solve(input: &str) -> (u32, u32) {
    let (mut a, mut b) = parse_input(input);
    a.sort();
    b.sort();

    let part1 = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b));

    let mut part2 = 0;
    let mut i = 0;
    let mut j = 0;
    let b_len = b.len();
    let a_len = a.len();
    // padd the arrays with 1 extra value to avoid having to check `j < b.len()` everywhere
    a.push(std::u32::MAX);
    b.push(std::u32::MAX);
    while i < a_len && j < b_len {
        let mut num_matches = 0;
        while b[j] < a[i] {
            j += 1;
        }
        while b[j] == a[i] {
            num_matches += 1;
            j += 1;
        }
        let cur_a_val = a[i];
        while a[i] == cur_a_val {
            part2 += cur_a_val * num_matches;
            i += 1
        }
    }

    (part1, part2)
}

pub fn day01() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 31);
    }
}
