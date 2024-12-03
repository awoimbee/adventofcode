const INPUT: &str = include_str!("../input/day02.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut parser = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::i32::<_, nom::error::Error<_>>,
        ),
    );

    let (remaining, parsed) = parser(input).expect("Could not parse input");
    assert_eq!(remaining.trim(), "");
    parsed
}

fn is_safe_level_interval(sign: &mut i32, diff: i32) -> bool {
    match (*sign, diff) {
        (-1, d) if -4 < d && d < 0 => true,
        (1, d) if 0 < d && d < 4 => true,
        (0, d) if 0 < d.abs() && d.abs() < 4 => {
            if d < 0 {
                *sign = -1;
            } else {
                *sign = 1;
            }
            true
        }
        _ => false,
    }
}

fn is_report_safe(report: &mut Vec<i32>, did_rm_level: bool) -> (bool, bool) {
    let mut sign = 0;
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if !is_safe_level_interval(&mut sign, diff) {
            if did_rm_level {
                return (false, false);
            }
            let report_i = report.remove(i);
            if is_report_safe(report, true).1 {
                return (false, true);
            }
            let report_i_plus_one = report[i];
            report[i] = report_i; // add report[i] back and remove report[i+1]
            if is_report_safe(report, true).1 {
                return (false, true);
            }
            if i > 0 {
                report[i - 1] = report[i];
                report[i] = report_i_plus_one;
                if is_report_safe(report, true).1 {
                    return (false, true);
                }
            }
            return (false, false);
        }
    }
    (!did_rm_level, true)
}

fn solve(input: &str) -> (u32, u32) {
    let table = parse_input(input);

    let mut num_safe_reports_p1 = 0;
    let mut num_safe_reports_p2 = 0;

    for mut report in table.into_iter() {
        let (safe_p1, safe_p2) = is_report_safe(&mut report, false);
        num_safe_reports_p1 += safe_p1 as u32;
        num_safe_reports_p2 += safe_p2 as u32;
    }

    (num_safe_reports_p1, num_safe_reports_p2)
}

pub fn day02() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    #[test]
    fn test_is_report_safe() {
        let reports = vec![
            vec![48, 46, 47, 49, 51, 54, 56],
            vec![1, 1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 5],
            vec![5, 1, 2, 3, 4, 5],
            vec![1, 4, 3, 2, 1],
            vec![1, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 3],
            vec![9, 8, 7, 6, 7],
            vec![7, 10, 8, 10, 11],
            vec![29, 28, 27, 25, 26, 25, 22, 20],
        ];
        for report in reports {
            assert!(
                is_report_safe(&mut (report.clone()), false).1,
                "report {report:?} failed"
            );
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 4);
        assert_eq!(solve("1 1 2\n2 6 1").1, 2);
    }
}
