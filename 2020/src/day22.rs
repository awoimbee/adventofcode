use fnv::FnvHasher;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("../input/day22.txt");

type Decks = [VecDeque<u8>; 2];

// top of deck == front of vecdeque
fn parse(input: &str) -> Decks {
    let vec: Vec<_> = input
        .split("\n\n")
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|card| card.parse().unwrap())
                .collect()
        })
        .collect();
    let res: Decks = vec.try_into().unwrap();
    res
}

fn part1(mut decks: Decks) -> u64 {
    while !decks[0].is_empty() && !decks[1].is_empty() {
        let p1_card = decks[0].pop_front().unwrap();
        let p2_card = decks[1].pop_front().unwrap();
        if p1_card > p2_card {
            decks[0].push_back(p1_card);
            decks[0].push_back(p2_card);
        } else {
            decks[1].push_back(p2_card);
            decks[1].push_back(p1_card);
        }
    }
    let winner_deck = if decks[0].is_empty() {
        &mut decks[1]
    } else {
        &mut decks[0]
    };
    winner_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as u64 * (*card) as u64)
        .sum()
}

fn decks_hash(decks: &Decks) -> u64 {
    let mut hasher = FnvHasher::default();
    decks.hash(&mut hasher);
    hasher.finish()
}

fn recursive_combat(decks: &mut Decks) -> usize {
    let mut previous_states = HashSet::new();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        let hashed = decks_hash(decks);
        if previous_states.contains(&hashed) {
            return 0;
        }
        previous_states.insert(hashed);

        let cards_drawn = [decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];

        let winner =
            if cards_drawn[0] <= decks[0].len() as u8 && cards_drawn[1] <= decks[1].len() as u8 {
                let mut new_decks = decks.clone();
                new_decks[0].resize(cards_drawn[0] as usize, 0);
                new_decks[1].resize(cards_drawn[1] as usize, 0);
                recursive_combat(&mut new_decks)
            } else {
                if cards_drawn[0] > cards_drawn[1] {
                    0
                } else {
                    1
                }
            };
        let looser = (winner + 1) % 2;
        decks[winner].push_back(cards_drawn[winner]);
        decks[winner].push_back(cards_drawn[looser]);
    }
    if decks[0].is_empty() {
        1
    } else {
        0
    }
}

fn part2(mut decks: Decks) -> u64 {
    let winner = recursive_combat(&mut decks);
    decks[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as u64 * (*card) as u64)
        .sum()
}

pub fn day22() -> (String, String) {
    let decks = parse(INPUT);

    let p1 = part1(decks.clone());
    let p2 = part2(decks);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "Player 1:\n9\n2\n6\n3\n1\n\n",
        "Player 2:\n5\n8\n4\n7\n10\n"
    );

    #[test]
    fn test_p1_example() {
        let decks = parse(TEST_INPUT);
        assert!(part1(decks) == 306);
    }
    #[test]
    fn test_p1_challenge() {
        let decks = parse(INPUT);
        assert!(part1(decks) == 33559);
    }
    #[test]
    fn test_p2_example() {
        let decks = parse(TEST_INPUT);
        assert!(part2(decks) == 291);
    }
    #[test]
    fn test_p2_challenge() {
        let decks = parse(INPUT);
        assert!(part2(decks) == 32789);
    }
}
