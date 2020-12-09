const INPUT: &[u8] = include_bytes!("../input/day01.txt");

fn part_1(mut numbers: Vec<usize>) -> String {
    while let Some(nb) = numbers.pop() {
        let needed = 2020 - nb;
        if numbers.contains(&needed) {
            return format!("Part 1: {}", nb * needed);
        }
    }
    unreachable!("Part 1: Not found !");
}

fn part_2(mut numbers: Vec<usize>) -> String {
    while let Some(nb0) = numbers.pop() {
        if nb0 > 2020 {
            continue;
        };
        let max = 2020 - nb0;
        let mut numbers_slice = numbers.as_slice();
        while let Some(&nb1) = numbers_slice.last() {
            numbers_slice = &numbers_slice[0..numbers_slice.len() - 1];
            if nb1 > max {
                continue;
            };
            let needed = 2020 - nb0 - nb1;
            if numbers_slice.contains(&needed) {
                return format!("Part 2: {}", nb0 * nb1 * needed);
            }
        }
    }
    unreachable!("Part 2: Not found !");
}

pub fn day01() -> (String, String) {
    let numbers: Vec<usize> = INPUT
        .split(|&c| c == b'\n')
        .filter(|s| !s.is_empty())
        .map(|e| unsafe { std::str::from_utf8_unchecked(e).parse().unwrap() })
        .filter(|&nb| nb <= 2020)
        .collect();

    let p1 = part_1(numbers.clone());
    let p2 = part_2(numbers);
    (p1, p2)
}
