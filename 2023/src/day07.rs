const INPUT: &str = include_str!("../input/day07.txt");

#[derive(Eq, Debug, Clone, Ord)]
struct CardHand {
    pub bid: i64,
    pub cards: u64,
    pub kind: u32,
}

impl std::cmp::PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == other.cards
    }
}

impl std::cmp::PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind != other.kind {
            Some(self.kind.cmp(&other.kind))
        } else {
            Some(self.cards.cmp(&other.cards))
        }
    }
}

fn solve_p1(input: &str) -> i64 {
    let mut hands: Vec<CardHand> = Vec::new();

    for hand_str in input.lines() {
        let (cards_str, bid_str) = hand_str.split_once(' ').unwrap();
        let mut cards_values: [u8; 14] = [0; 14];
        let mut cards = 0u64;

        for card in cards_str.as_bytes().iter() {
            let card_value = match card {
                b'2'..=b'9' => card - b'2',
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => unreachable!(),
            };
            cards_values[card_value as usize] += 1;
            cards = (cards << 8) | card_value as u64;
        }

        cards_values.sort();
        cards_values.reverse();
        let hand_kind = match &cards_values[0..2] {
            [5, _] => 6, // five
            [4, _] => 5, // four
            [3, 2] => 4, // full house
            [3, _] => 3, // three of a kind
            [2, 2] => 2, // two pairs
            [2, _] => 1, // one pair
            _ => 0,      // one card
        };

        hands.push(CardHand {
            bid: bid_str.parse().unwrap(),
            kind: hand_kind,
            cards: cards,
        });
    }
    hands.sort();
    let mut p1 = 0;
    for (i, hand) in hands.iter().enumerate() {
        p1 += hand.bid * (i as i64 + 1);
    }

    p1
}

fn solve_p2(input: &str) -> i64 {
    let mut hands: Vec<CardHand> = Vec::new();

    for hand_str in input.lines() {
        let (cards_str, bid_str) = hand_str.split_once(' ').unwrap();
        let mut cards_values: [u8; 14] = [0; 14];
        let mut cards = 0u64;

        for card in cards_str.as_bytes().iter() {
            let card_value = match card {
                b'J' => 0,
                b'2'..=b'9' => card - b'1',
                b'T' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => unreachable!(),
            };
            cards_values[card_value as usize] += 1;
            cards = (cards << 8) | card_value as u64;
        }
        let num_jokers = cards_values[0];
        cards_values[0] = 0; // ignore jokers
        cards_values.sort();
        cards_values.reverse();
        cards_values[0] += num_jokers;
        let hand_kind = match &cards_values[0..2] {
            [5, _] => 6, // five
            [4, _] => 5, // four
            [3, 2] => 4, // full house
            [3, _] => 3, // three of a kind
            [2, 2] => 2, // two pairs
            [2, _] => 1, // one pair
            _ => 0,      // one card
        };

        hands.push(CardHand {
            bid: bid_str.parse().unwrap(),
            kind: hand_kind,
            cards: cards,
        });
    }
    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as i64 + 1);
    }

    sum
}

pub fn day07() -> (String, String) {
    let p1 = solve_p1(INPUT);
    let p2 = solve_p2(INPUT);

    // 247670564 too low

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_p1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_p2(TEST_INPUT), 5905);
    }
}
