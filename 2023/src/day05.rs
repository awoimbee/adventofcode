use std::ops::Add;

const INPUT: &str = include_str!("../input/day05.txt");

fn move_range<T: Ord + Copy + Add<Output = T>>(
    r: &std::ops::Range<T>,
    offset: T,
) -> std::ops::Range<T> {
    r.start + offset..r.end + offset
}

fn solve(input: &str) -> (i64, i64) {
    let mut categories = input.split("\n\n");
    let mut seeds_p1: Vec<i64> = categories
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let mut seeds_p2: Vec<_> = seeds_p1.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();

    let maps = categories.map(|cat| {
        let mut lines = cat.lines();
        lines.next();
        lines
            .map(|line| {
                line.split(' ')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });
    for map in maps {
        let mut mapped_seeds_p2 = Vec::with_capacity(seeds_p2.len() * 2);
        for line in map {
            let source_range = line[1]..line[1] + line[2];
            let diff = line[0] - line[1];
            for seed in &mut seeds_p1 {
                if source_range.contains(seed) {
                    // use negative numbers to indicate that the seed has already been mapped
                    *seed = -(*seed + diff);
                }
            }
            let mut tmp_seeds_p2 = Vec::with_capacity(seeds_p2.len() * 2);
            for seed_range in seeds_p2.into_iter() {
                if source_range.start < seed_range.end && seed_range.start < source_range.end {
                    let mut overlap = seed_range.start.max(source_range.start)
                        ..seed_range.end.min(source_range.end);
                    let before = seed_range.start..overlap.start;
                    let after = overlap.end..seed_range.end;

                    overlap = move_range(&overlap, diff);
                    mapped_seeds_p2.push(overlap);
                    if before.start < before.end {
                        tmp_seeds_p2.push(before);
                    }
                    if after.start < after.end {
                        tmp_seeds_p2.push(after);
                    }
                } else {
                    tmp_seeds_p2.push(seed_range.clone());
                }
            }
            seeds_p2 = tmp_seeds_p2;
        }
        seeds_p2.append(&mut mapped_seeds_p2);
        for s in &mut seeds_p1 {
            *s = s.abs();
        }
    }

    let p1 = seeds_p1.into_iter().fold(i64::MAX, |acc, x| acc.min(x));
    let p2 = seeds_p2
        .into_iter()
        .fold(i64::MAX, |acc, x| acc.min(x.start));

    (p1, p2)
}

pub fn day05() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 46);
    }
}
