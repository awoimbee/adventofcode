const INPUT: &str = include_str!("../input/day04.txt");

/// Returns the number of wins for each card.
fn get_card_wins(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|card| {
            let (_card_id, values) = card.split_once(": ").unwrap();
            let (win_nums, scratched_nums) = values.split_once(" | ").unwrap();
            let mut win_nums = win_nums
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            win_nums.sort_unstable();
            let mut scratched_nums = scratched_nums
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            scratched_nums.sort_unstable();

            let mut num_wins = 0;
            let mut win_num_id = 0;
            'scratched: for num in scratched_nums {
                while win_nums[win_num_id] < num {
                    win_num_id += 1;
                    if win_num_id == win_nums.len() {
                        break 'scratched;
                    }
                }
                if win_nums[win_num_id] == num {
                    num_wins += 1;
                }
            }
            num_wins
        })
        .collect()
}

fn solve_p1(wins: &[u32]) -> u32 {
    wins.iter()
        .map(|&wins| if wins == 0 { 0 } else { 1 << (wins - 1) })
        .sum()
}

fn solve_p2(wins: &[u32]) -> u32 {
    let mut cards_copies = vec![1; wins.len()];

    let mut i = 0;
    while i < cards_copies.len() {
        let num_wins = wins[i] as usize;
        for j in i + 1..(i + num_wins + 1).min(cards_copies.len()) {
            // We play all the replicas at the same time -> gain all the copies at the same time
            cards_copies[j] += cards_copies[i];
        }
        i += 1;
    }

    cards_copies.into_iter().sum()
}

fn solve(input: &str) -> (u32, u32) {
    let wins = get_card_wins(input);
    let p1 = solve_p1(&wins);
    let p2 = solve_p2(&wins);

    (p1, p2)
}

pub fn day04() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#};

    #[test]
    fn test_get_card_wins() {
        assert_eq!(get_card_wins(TEST_INPUT), vec![4, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_part_1() {
        let wins = get_card_wins(TEST_INPUT);
        assert_eq!(solve_p1(&wins), 13);
    }

    #[test]
    fn test_part_2() {
        let wins = get_card_wins(TEST_INPUT);
        assert_eq!(solve_p2(&wins), 30);
    }
}
