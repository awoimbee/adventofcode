use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::intrinsics::likely;

const INPUT: &str = include_str!("../input/day23.txt");

const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const ROOM2HALLWAY: [usize; 4] = [2, 4, 6, 8];
const EMPTY: u8 = u8::MAX;

#[inline]
const fn amphipod2inidex(a: impl Into<usize>) -> usize {
    a.into() - b'A' as usize
}
#[inline]
const fn index2amphipod(a: impl Into<u8>) -> u8 {
    a.into() + b'A'
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map {
    hallway: [u8; 11],
    rooms: [[u8; 2]; 4],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    map: Map,
    cost: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl State {
    pub fn new(pos: u32, cost: u32) -> Self {
        Self { pos, cost }
    }
}

impl Map {
    #[inline]
    fn room_is_valid(&self, room: impl Into<usize>) -> bool {
        let i = room.into();
        let r = self.rooms[i];
        if amphipod2inidex(r[0]) != i || amphipod2inidex(r[1]) != i {
            return false;
        }
        return true
    }

    pub fn is_done(&self) -> bool {
    (0..self.rooms.len()).all(|i| self.room_is_valid(i))
    }

    fn expand(&self) -> Vec<Map> {
        // let amphipods = self.hallway.iter().enumerate().filter(|(i,&a)| a != EMPTY)
        // .map(|(i, a)| i)
        // .chain(
        //     self.rooms.iter().enumerate().flat_map(|(i,r)| r.iter().filter(|&&a| a != EMPTY)),
        // );

        let mut result = Vec::new();

        for (i, r) in self.rooms.iter().enumerate() {
            if self.room_is_valid(i) {
                continue;
            }

            for j in 0..r.len() {
                if r[j] == EMPTY {
                    continue;
                }

                let mut new_rooms = self.rooms.clone();
                new_rooms[i] = [index2amphipod(ROOM2HALLWAY[j]), index2amphipod(ROOM2HALLWAY[j])];
                let mut new_hallway = self.hallway.clone();
                new_hallway[ROOM2HALLWAY[j]] = EMPTY;
                result.push(Map {
                    hallway: new_hallway,
                    rooms: new_rooms,
                });
            }



            // let mut new_map = self.clone();
            // new_map.rooms[i][0] = ROOM2AMPHIPOD[i];
            // new_map.rooms[i][1] = ROOM2AMPHIPOD[i];
            // result.push(new_map);
        }
        result
    }

    pub fn djikstra(&self) -> State {
        // TODO: This is from day 15
        let mut open = BinaryHeap::new();
        let mut cost_map = vec![u32::MAX; self.data.len()];
        let mut best_solution = None;
        open.push(State::new(0, 0));
        while let Some(state) = open.pop() {
            let closed_cost = cost_map[state.pos as usize];
            if state.cost >= closed_cost {
                continue;
            }
            cost_map[state.pos as usize] = state.cost;
            if state.pos == (self.data.len() - 1) as u32 {
                best_solution = Some(state);
                continue;
            }
            // expand
            let (x, y) = self.pos2d(state.pos);
            // right
            if likely(x + 1 < self.width) {
                let pos = state.pos + 1;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost));
            }
            // down
            if likely(y + 1 < self.height) {
                let pos = state.pos + self.width;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost));
            }
            // left (likely bad)
            if likely(x > 0) {
                let pos = state.pos - 1;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost));
            }
            // up (likely bad)
            if likely(y > 0) {
                let pos = state.pos - self.width;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost));
            }
        }
        best_solution.unwrap()
    }
}

pub fn day23() -> (String, String) {
    let mut parsed = Map::from_str(INPUT);
    let part1 = parsed.djikstra().cost;
    let part2 = "";

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_done() {
        const INPUT: &str = "
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let parsed = Map::from_str(TEST_INPUT);
        let final_state = parsed.djikstra();
        assert_eq!(final_state.cost, 40);
    }
    #[test]
    fn test_part_1() {
        let parsed = Map::from_str(INPUT);
        let final_state = parsed.djikstra();
        assert_eq!(final_state.cost, 373);
    }

    #[test]
    fn test_part_2_test_input() {
        let mut parsed = Map::from_str(TEST_INPUT);
        parsed.extend_5_times();
        parsed._print();
        let final_state = parsed.djikstra();
        assert_eq!(final_state.cost, 315);
    }

    #[test]
    #[cfg_attr(not(feature = "expensive_tests"), ignore)]
    fn test_part_2() {
        let mut parsed = Map::from_str(INPUT);
        parsed.extend_5_times();
        let final_state = parsed.djikstra();
        assert_eq!(final_state.cost, 2868);
    }
}
