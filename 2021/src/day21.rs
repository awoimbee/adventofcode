use fnv::FnvHashMap;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    error::Error,
    sequence::{preceded, terminated},
};

const INPUT: &str = include_str!("../input/day21.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: u8,
    score: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Game {
    players: [Player; 2],
    // Only used in part 2:
    whosturn: u8,
}

// out of 3 rolls
// 1 way to get 3: 1,1,1
// 3 ways to get 4: 1,1,2 1,2,1 2,1,1
// ...
const HISTOGRAM: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl Game {
    pub fn from_str(s: &str) -> Self {
        let mut parser = preceded::<&str, _, _, Error<&str>, _, _>(
            tag("Player "),
            nom::sequence::pair(
                terminated(complete::u32, tag(" starting position: ")),
                complete::u8,
            ),
        );
        if let Some((p1, p2)) = s
            .lines()
            .map(|l| {
                let raw_data = parser(l).expect("Invalid input");
                // extrernally positions are 1-10, internally are 0-9
                Player {
                    position: raw_data.1 .1 - 1,
                    score: 0,
                }
            })
            .collect_tuple()
        {
            Self {
                players: [p1, p2],
                whosturn: 0,
            }
        } else {
            panic!("Invalid number of players !");
        }
    }

    pub fn part1(&mut self) -> u32 {
        let mut it_dice = (1..=100).cycle().enumerate();
        let mut it_players = (0..self.players.len()).cycle();

        for p in it_players.by_ref() {
            let player = &mut self.players[p];
            let roll: u16 = it_dice.by_ref().take(3).map(|(_, val)| val).sum();
            player.position = ((player.position as u16 + roll) % 10) as u8;
            player.score += (player.position + 1) as u16;
            if player.score >= 1000 {
                break;
            }
        }
        let loosing_player = it_players.next().unwrap();
        let dice_rolls = it_dice.next().unwrap().0 as u32;
        self.players[loosing_player].score as u32 * dice_rolls
    }

    fn recurse_part2(state: Self, cache: &mut FnvHashMap<Self, (u64, u64)>) -> (u64, u64) {
        if state.players[0].score >= 21 {
            return (1, 0);
        } else if state.players[1].score >= 21 {
            return (0, 1);
        }

        let mut wins = (0, 0);
        for &(roll, freq) in HISTOGRAM.iter() {
            let mut next_state = state.clone();
            let player = &mut next_state.players[next_state.whosturn as usize];
            player.position = (player.position + roll) % 10;
            player.score += (player.position + 1) as u16;
            next_state.whosturn = (next_state.whosturn + 1) % 2;
            let (p1, p2) = match cache.get(&next_state) {
                Some(wins) => *wins,
                None => {
                    let wins = Self::recurse_part2(next_state.clone(), cache);
                    cache.insert(next_state, wins);
                    wins
                }
            };
            wins.0 += p1 * freq;
            wins.1 += p2 * freq;
        }
        wins
    }

    pub fn part2(&mut self) -> u64 {
        let mut cache = FnvHashMap::default();
        let results = Self::recurse_part2(self.clone(), &mut cache);
        if results.0 > results.1 {
            results.0
        } else {
            results.1
        }
    }
}

pub fn day21() -> (String, String) {
    let mut parsed = Game::from_str(INPUT);
    // println!("{:?}", parsed);
    let p1 = parsed.clone().part1().to_string();
    let p2 = parsed.part2().to_string();
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4\n\
        Player 2 starting position: 8";

    #[test]
    fn test_parsing() {
        let parsed = Game::from_str(TEST_INPUT);

        assert_eq!(parsed.players.len(), 2);
        assert_eq!(parsed.players[0].position, 3);
        assert_eq!(parsed.players[1].position, 7);
        assert_eq!(parsed.players[0].score, 0);
        assert_eq!(parsed.players[1].score, 0);
    }

    #[test]
    fn test_part1() {
        let mut parsed = Game::from_str(TEST_INPUT);
        assert_eq!(parsed.part1(), 739785);
    }

    #[test]
    fn test_part2() {
        let mut parsed = Game::from_str(TEST_INPUT);
        assert_eq!(parsed.part2(), 444356092776315);
    }
}
