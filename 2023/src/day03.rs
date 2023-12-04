const INPUT: &str = include_str!("../input/day03.txt");

fn is_symbol(c: &u8) -> bool {
    !c.is_ascii_digit() && *c != b'.'
}

fn parse_num_p2(line: &[u8], coord: usize) -> u32 {
    let mut start = coord;
    while start != 0 && line[start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = coord;
    while end < line.len() && line[end].is_ascii_digit() {
        end += 1;
    }
    unsafe { std::str::from_utf8_unchecked(&line[start..end]) }
        .parse()
        .unwrap()
}

fn solve(input: &str) -> (u32, u32) {
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    for i in 0..lines.len() {
        let mut j = 0;
        'chars: while j < lines[i].len() {
            if lines[i][j].is_ascii_digit() {
                let start = j;
                while j < lines[i].len() && lines[i][j].is_ascii_digit() {
                    j += 1;
                }
                let end = j;
                let adj_start = start.saturating_sub(1);
                let adj_end = (end).min(lines[i].len() - 1);
                if is_symbol(&lines[i][adj_start])
                    || is_symbol(&lines[i][adj_end])
                    || (i > 0 && lines[i - 1][adj_start..=adj_end].iter().any(is_symbol))
                    || (i < lines.len() - 1
                        && lines[i + 1][adj_start..=adj_end].iter().any(is_symbol))
                {
                    let num = unsafe { std::str::from_utf8_unchecked(&lines[i][start..end]) }
                        .parse::<u32>()
                        .unwrap();
                    sum_p1 += num;
                }
                continue 'chars;
            } else if lines[i][j] == b'*' {
                let mut coord_0 = None;
                let mut coord_1 = None;

                for u in i.saturating_sub(1)..=(i + 1).min(lines.len() - 1) {
                    let mut v = j.saturating_sub(1);
                    while v <= (j + 1).min(lines[i].len() - 1) {
                        if lines[u][v].is_ascii_digit() {
                            if coord_0.is_none() {
                                coord_0 = Some((u, v));
                            } else if coord_1.is_none() {
                                coord_1 = Some((u, v));
                            } else {
                                j += 1;
                                continue 'chars;
                            }
                            while v < lines[i].len() && lines[u][v].is_ascii_digit() {
                                v += 1;
                            }
                            continue;
                        }
                        v += 1;
                    }
                }
                if coord_0.is_some() && coord_1.is_some() {
                    let num1 = parse_num_p2(lines[coord_0.unwrap().0], coord_0.unwrap().1);
                    let num2 = parse_num_p2(lines[coord_1.unwrap().0], coord_1.unwrap().1);
                    sum_p2 += num1 * num2;
                }
            }
            j += 1;
        }
    }

    (sum_p1, sum_p2)
}

pub fn day03() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 467835);
    }
}
