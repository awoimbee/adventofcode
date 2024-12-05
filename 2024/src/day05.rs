use std::cmp::Ordering;

use nom::Parser;

const INPUT: &str = include_str!("../input/day05.txt");

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parser = nom::sequence::separated_pair(
        nom::multi::separated_list1(
            nom::character::complete::newline::<_, nom::error::Error<_>>,
            nom::sequence::separated_pair(
                nom::character::complete::u32,
                nom::bytes::complete::tag("|"),
                nom::character::complete::u32,
            ),
        ),
        nom::bytes::complete::tag("\n\n"),
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::separated_list1(
                nom::bytes::complete::tag(","),
                nom::character::complete::u32,
            ),
        ),
    );

    let (remaining, parsed) = parser.parse(input).expect("Could not parse input");
    assert_eq!(remaining.trim(), "");

    parsed
}

fn solve(input: &str) -> (u32, u32) {
    let (raw_ordering_rules, updates) = parse(input);

    let mut ordering_rules = [const { Vec::<u32>::new() }; 100];
    for or in raw_ordering_rules {
        ordering_rules[or.1 as usize].push(or.0);
    }
    ordering_rules.iter_mut().for_each(|vec| vec.sort());

    let mut p1 = 0;
    let mut p2 = 0;
    for up in updates {
        let mut up2 = up.clone();

        up2.sort_unstable_by(|x, y| {
            let required_prefixes = &ordering_rules[*y as usize];
            if required_prefixes.binary_search(x).is_ok() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if up == up2 {
            log::debug!("Correct order for {:?}, adding {}", up, up[up.len() / 2]);
            p1 += up[up.len() / 2];
        } else {
            log::debug!(
                "Wring order for {:?} -> {:?}, adding {}",
                up,
                up2,
                up2[up2.len() / 2]
            );
            p2 += up2[up2.len() / 2];
        }
    }

    (p1, p2)
}

pub fn day05() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT).0, 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT).1, 123);
    }
}
