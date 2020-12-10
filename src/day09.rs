const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day09.txt")) };
const PREAMBLE: usize = 25;

fn parse() -> impl Iterator<Item = usize> {
    INPUT
        .split('\n')
        .filter(|s| !str::is_empty(s))
        .map(|s| s.parse().unwrap())
}

fn try_sum(preamble: &[usize], target: usize) -> bool {
    for (idx, &i0) in preamble.iter().enumerate() {
        if i0 > target {
            continue;
        }
        for &i1 in &preamble[idx + 1..] {
            if i0 + i1 == target {
                return true;
            }
        }
    }
    false
}

fn part_1(numbers: &[usize]) -> usize {
    let mut idx = PREAMBLE;
    while idx < numbers.len() {
        let nb = numbers[idx];
        if !try_sum(&numbers[idx - PREAMBLE..idx], nb) {
            return nb;
        }
        idx += 1;
    }
    unreachable!();
}

fn part_2(numbers: &[usize], p1_result: usize) -> usize {
    let mut start = 0;
    let mut end = 1;
    let mut running_sum = numbers[start];

    while start < numbers.len() {
        while end < numbers.len() {
            running_sum += numbers[end];
            if running_sum == p1_result {
                let (min, max) = {
                    let mut mi = std::usize::MAX;
                    let mut ma = std::usize::MIN;
                    for &i in &numbers[start..=end] {
                        if i < mi {
                            mi = i;
                        } else if i > ma {
                            ma = i;
                        }
                    }
                    (mi, ma)
                };
                return min + max;
            }
            end += 1;
        }
        start += 1;
        end = start + 1;
        running_sum = numbers[start];
    }
    unreachable!();
}

pub fn day09() -> (String, String) {
    let numbers = parse().collect::<Vec<_>>();
    let p1 = part_1(&numbers);
    let p2 = part_2(&numbers, p1);
    (format!("{}", p1), format!("{}", p2))
}
