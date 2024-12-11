use core::str;
use std::collections::HashMap;


const INPUT: &str = include_str!("../input/day11.txt");

fn split_even_digits(num: u64) -> Option<(u64, u64)> {
    let mut digit_count = 0;
    let mut n = num;
    while n > 0 {
        n /= 10;
        digit_count += 1;
    }
    if digit_count % 2 != 0 {
        return None;
    }
    let divisor = match digit_count {
        2 => 10,
        4 => 100,
        6 => 1_000,
        8 => 10_000,
        10 => 100_000,
        12 => 1_000_000,
        _ => unreachable!(), // _ => unsafe { unreachable_unchecked()}
    };
    let left = num / divisor;
    let right = num % divisor;

    Some((left, right))
}

fn stone(mut val: u64, iter: usize, shortcut: &mut HashMap<(u64, usize), u64>) -> u64 {
    let mut res = 1;
    for i in 0..iter {
        if val == 0 {
            val = 1;
        } else if let Some((a, b)) = split_even_digits(val) {
            val = a;
            let key = (b, (iter - (i + 1)) );
            if let Some(r) = shortcut.get(&key) {
                res += r;
            } else {
                let r = stone(b, iter - (i + 1), shortcut);
                res += r;
                shortcut.insert(key, r);
            }
        } else {
            val *= 2024;
        }
    }
    res
}

fn solve(input: &str) -> (u64, u64) {
    let parsed: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();

    let p1 = parsed.iter().map(|s| stone(*s, 25, &mut cache)).sum();
    let p2 = parsed.iter().map(|s| stone(*s, 75, &mut cache)).sum();
    (p1, p2)
}

pub fn day11() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_split_even_digits() {
        init();
        assert_eq!(split_even_digits(10), Some((1, 0)));
        assert_eq!(split_even_digits(902834), Some((902, 834)));
        assert_eq!(split_even_digits(90283), None);
    }

    #[test]
    fn test_stone() {
        init();

        assert_eq!(stone(125, 1, &mut HashMap::new()), 1);
        assert_eq!(stone(125, 2, &mut HashMap::new()), 2);
        assert_eq!(stone(125, 3, &mut HashMap::new()), 2);
        assert_eq!(stone(125, 4, &mut HashMap::new()), 3);
        assert_eq!(stone(125, 5, &mut HashMap::new()), 5);
    }

    #[test]
    fn test_part_1() {
        init();
        assert_eq!(solve("125 17").0, 55312);
    }
}
