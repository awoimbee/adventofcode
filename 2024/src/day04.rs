const INPUT: &str = include_str!("../input/day04.txt");

fn solve(input: &str) -> (u32, u32) {
    let table = input.as_bytes();
    let width = table.iter().position(|c| *c == b'\n').unwrap() + 1;
    let w = width as isize;

    let p1_directions = [
        [0, 1, 2, 3],
        [0, -1, -2, -3],
        [0, w, 2 * w, 3 * w],
        [0, -w, -2 * w, -3 * w],
        [0, w + 1, 2 * w + 2, 3 * w + 3],
        [0, w - 1, 2 * w - 2, 3 * w - 3],
        [0, -w - 1, -2 * w - 2, -3 * w - 3],
        [0, -w + 1, -2 * w + 2, -3 * w + 3],
    ];
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for i in 0..table.len() {
        let is = i as isize;
        if table[is as usize] == b'\n' {
            continue;
        };
        if table[is as usize] == b'X' {
            for dir in p1_directions.iter() {
                if is + dir[3] < 0 || is + dir[3] >= table.len() as isize {
                    continue;
                }
                let to_match = dir.map(|d| unsafe { *table.get_unchecked((is + d) as usize) });
                if to_match == "XMAS".as_bytes() {
                    sum_p1 += 1;
                }
            }
        }
        if is < (table.len() as isize) - 2 - 2 * w {
            let a = [table[i], table[i + 1 + width], table[i + 2 + 2 * width]];
            let b = [table[i + 2], table[i + 1 + width], table[i + 2 * width]];

            if (a == "MAS".as_bytes() || a == "SAM".as_bytes())
                && (b == "MAS".as_bytes() || b == "SAM".as_bytes())
            {
                sum_p2 += 1;
            }
        }
    }

    (sum_p1, sum_p2)
}

pub fn day04() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 9);
    }
}
