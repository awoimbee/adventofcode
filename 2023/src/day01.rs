const INPUT: &str = include_str!("../input/day01.txt");

const fn add_num_to_tmp(num: u32, tmp: u32) -> u32 {
    if tmp == 0 {
        num * 10 + num
    } else {
        tmp / 10 * 10 + num
    }
}

fn solve(input: &str) -> (u32, u32) {
    const TEXT_DIGITS: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    for line in input.lines() {
        let mut tmp_p1 = 0;
        let mut tmp_p2 = 0;
        let bline = line.as_bytes();

        for i in 0..bline.len() {
            if bline[i].is_ascii_digit() {
                let num = (bline[i] - b'0') as u32;
                tmp_p1 = add_num_to_tmp(num, tmp_p1);
                tmp_p2 = add_num_to_tmp(num, tmp_p2);
            } else {
                for (j, t) in TEXT_DIGITS.iter().enumerate() {
                    if bline[i..].starts_with(t) {
                        let num = (j + 1) as u32;
                        tmp_p2 = add_num_to_tmp(num, tmp_p2);
                        break;
                    }
                }
            }
        }
        sum_p1 += tmp_p1;
        sum_p2 += tmp_p2;
    }

    (sum_p1, sum_p2)
}

pub fn day01() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_P1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
    const TEST_INPUT_P2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT_P1).0, 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT_P2).1, 281);
    }
}
