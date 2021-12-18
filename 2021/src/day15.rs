use fnv::FnvHashMap;
use priority_queue::priority_queue::PriorityQueue;
use std::intrinsics::likely;

const INPUT: &str = include_str!("../input/day15.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: u32,
    cost: u32,
}

impl State {
    pub fn new(pos: u32, cost: u32) -> Self {
        Self { pos, cost }
    }
}

struct Map {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

fn cost2prio(elem: u32) -> u32 {
    u32::MAX - elem
}

impl Map {
    pub fn from_str(input: &str) -> Self {
        let width = input.find('\n').expect("invalid map") as u32;
        let data: Vec<u8> = input
            .split_terminator('\n')
            .flat_map(|line| line.bytes().map(|c| c - b'0'))
            .collect();
        let height = data.len() as u32 / width;
        Self {
            width,
            height,
            data,
        }
    }

    pub fn extend_5_times(&mut self) {
        let mut new_data = Vec::with_capacity(self.data.len() * 25);
        for y in 0..(self.height * 5) {
            for x in 0..(self.width * 5) {
                let d = self.data[self.pos1d(x % self.width, y % self.height)];
                let mut d = d as u32 + x / self.width + y / self.height;
                if d > 9 {
                    d %= 9;
                }
                new_data.push(d as u8);
            }
        }
        self.data = new_data;
        self.width *= 5;
        self.height *= 5;
    }

    fn pos2d(&self, pos1d: u32) -> (u32, u32) {
        let x = pos1d % self.width;
        let y = pos1d / self.width;
        (x as u32, y as u32)
    }
    fn pos1d(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn djikstra(&self) -> State {
        let mut open = PriorityQueue::new();
        let mut closed = FnvHashMap::<u32, u32>::default();
        let mut best_solution = None;

        open.push(State::new(0, 0), cost2prio(0));
        while let Some((state, _)) = open.pop() {
            // println!("{}", open.len());
            if let Some(&closed_cost) = closed.get(&state.pos) {
                if state.cost >= closed_cost {
                    continue;
                }
            }
            closed.insert(state.pos, state.cost);

            if state.pos == (self.data.len() - 1) as u32 {
                println!("found solution: {}", state.cost);
                best_solution = Some(state);
                continue;
            }
            // expand
            let (x, y) = self.pos2d(state.pos);
            // let cost = state.cost + self.data[state.pos as usize] as u32;
            if likely(x + 1 < self.width) {
                let pos = state.pos + 1;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost), cost2prio(cost));
            }
            if likely(x > 0) {
                let pos = state.pos - 1;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost), cost2prio(cost));
            }
            if likely(y + 1 < self.height) {
                let pos = state.pos + self.width;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost), cost2prio(cost));
            }
            if likely(y > 0) {
                let pos = state.pos - self.width;
                let cost = state.cost + self.data[pos as usize] as u32;
                open.push(State::new(pos, cost), cost2prio(cost));
            }
        }
        best_solution.unwrap()
    }

    fn _print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[self.pos1d(x, y)]);
            }
            println!();
        }
        println!();
    }
}

pub fn day15() -> (String, String) {
    let mut parsed = Map::from_str(INPUT);
    let part1 = parsed.djikstra();
    parsed.extend_5_times();
    let part2 = parsed.djikstra();
    let part1 = part1.cost;
    let part2 = part2.cost;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1_test_input() {
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
