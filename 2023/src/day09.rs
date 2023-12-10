const INPUT: &str = include_str!("../input/day09.txt");

fn extrapolate_history(numbers: &[i64]) -> (i64, i64) {
    let differences = numbers
        .windows(2)
        .map(|chunk| chunk[1] - chunk[0])
        .collect::<Vec<_>>();
    let last_num = *numbers.last().unwrap();
    let first_num = *numbers.first().unwrap();
    if differences.iter().all(|diff| *diff == 0) {
        (first_num, last_num)
    } else {
        let (before_diff, after_diff) = extrapolate_history(&differences);
        (first_num - before_diff, last_num + after_diff)
    }
}

fn solve(input: &str) -> (i64, i64) {
    let report = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|val| val.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    report
        .iter()
        .map(|history| extrapolate_history(&history))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

pub fn day09() -> (String, String) {
    let (p2, p1) = solve(INPUT);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "#};

    #[test]
    fn test_extrapolate_history_after() {
        assert_eq!(extrapolate_history(&[0, 0, 0]).1, 0);
        assert_eq!(extrapolate_history(&[2, 2]).1, 2);
        assert_eq!(extrapolate_history(&[0, 2, 4, 6]).1, 8);
        assert_eq!(extrapolate_history(&[3, 3, 5, 9, 15]).1, 23);
    }

    #[test]
    fn test_extrapolate_history_before() {
        assert_eq!(extrapolate_history(&[0, 0, 0]).0, 0);
        assert_eq!(extrapolate_history(&[2, 2]).0, 2);
        assert_eq!(extrapolate_history(&[0, 2, 4, 6]).0, -2);
        assert_eq!(extrapolate_history(&[3, 3, 5, 9, 15]).0, 5);

        assert_eq!(extrapolate_history(&[10, 13, 16, 21, 30, 45]).0, 5);
        assert_eq!(extrapolate_history(&[0, 3, 6, 9, 12, 15]).0, -3);
        assert_eq!(extrapolate_history(&[1, 3, 6, 10, 15, 21]).0, 0);
    }

    #[test]
    fn test_solve_p1() {
        assert_eq!(solve(TEST_INPUT).1, 114);
    }

    #[test]
    fn test_solve_p2() {
        assert_eq!(solve(TEST_INPUT).0, 2);
    }
}
