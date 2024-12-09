use log::debug;

const INPUT: &str = include_str!("../input/day07.txt");

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut parser = nom::multi::separated_list1(
        nom::character::complete::newline::<_, nom::error::Error<_>>,
        nom::sequence::separated_pair(
            nom::character::complete::u64,
            nom::bytes::complete::tag(": "),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(" "),
                nom::character::complete::u64,
            ),
        ),
    );

    let (remaining, parsed) = parser(input).expect("Could not parse input");
    assert_eq!(remaining.trim(), "");

    parsed
}

fn concat(a: u64, b: u64) -> u64 {
    let mut digits_in_b = 0;
    let mut b_copy = b;
    while b_copy > 0 {
        b_copy /= 10;
        digits_in_b += 1;
    }
    a * 10_u64.pow(digits_in_b) + b
}

fn try_compute_p2(nums: &[u64]) -> Vec<u64> {
    let len = nums.len();
    let a = &nums[..len - 1];
    let b = nums[len - 1];
    if a.is_empty() {
        vec![b]
    } else {
        let resolved_a = try_compute_p2(a);
        let mut res = Vec::with_capacity(3 * resolved_a.len());
        res.extend(resolved_a.iter().map(|x| x * b));
        res.extend(resolved_a.iter().map(|x| x + b));
        res.extend(resolved_a.iter().map(|x| concat(*x, b)));
        res
    }
}

fn try_compute_p1(nums: &[u64]) -> Vec<u64> {
    let len = nums.len();
    let a = &nums[..len - 1];
    let b = nums[len - 1];
    if a.is_empty() {
        vec![b]
    } else {
        let resolved_a = try_compute_p1(a);
        let mut res = Vec::with_capacity(2 * resolved_a.len());
        res.extend(resolved_a.iter().map(|x| x * b));
        res.extend(resolved_a.iter().map(|x| x + b));
        res
    }
}

fn solve(input: &str) -> (u64, u64) {
    let parsed = parse(input);

    let mut p1 = 0;
    let mut p2 = 0;
    for (res, nums) in parsed {
        let results_1 = try_compute_p1(&nums);
        debug!("1: {} in {:?} ?", res, results_1);
        if results_1.contains(&res) {
            p1 += res;
            p2 += res;
        } else {
            let results_2 = try_compute_p2(&nums);
            debug!("2: {} in {:?} ?", res, results_2);
            if results_2.contains(&res) {
                p2 += res;
            }
        }
    }

    (p1, p2)
}

pub fn day07() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part_1() {
        init();
        assert_eq!(solve(TEST_INPUT).0, 3749);
    }

    #[test]
    fn test_part_2() {
        init();
        assert_eq!(solve(TEST_INPUT).1, 11387);
    }
}
