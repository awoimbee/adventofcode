use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day23.txt");

const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const BUF_WEIGHT: [i64; 7] = [0, 1, 3, 5, 7, 9, 10];

fn weight(c: u8) -> i64 {
    WEIGHT[(c - b'A') as usize]
}

fn buf_traversal_cost(i: usize, j: usize, c: u8) -> i64 {
    (BUF_WEIGHT[i] - BUF_WEIGHT[j]).abs() * WEIGHT[(c - b'A') as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    rooms: [Vec<u8>; 4],
    buffer: [u8; 7],
}

impl State {
    fn from_input(input: &str) -> Self {
        let input = input[14..].as_bytes();
        let i = input;
        let buffer = [i[1], i[2], i[4], i[6], i[8], i[10], i[11]];
        let rooms = [
            vec![i[18], i[32]],
            vec![i[20], i[34]],
            vec![i[22], i[36]],
            vec![i[24], i[38]],
        ];
        Self { rooms, buffer }
    }

    fn final_state() -> Self {
        State {
            rooms: [
                vec![b'A', b'A'],
                vec![b'B', b'B'],
                vec![b'C', b'C'],
                vec![b'D', b'D'],
            ],
            buffer: [b'.'; 7],
        }
    }

    fn is_valid_room(&self, i: usize) -> bool {
        self.rooms[i].iter().all(|&c| i == (c - b'A') as usize)
    }

    fn entry_cost(&self, i: usize) -> i64 {
        (4 - self.rooms[i].len()) as i64 * WEIGHT[i]
    }

    fn exit_cost(&self, i: usize, c: u8) -> i64 {
        (4 - self.rooms[i].len()) as i64 * weight(c)
    }

    fn transition_room_to_buffer(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..4 {
            if self.is_valid_room(i) {
                continue;
            }
            let mut next = self.clone();
            let c = next.rooms[i].pop().unwrap();
            for j in (0..=i + 1).rev() {
                let cost = buf_traversal_cost(j, i + 1, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == b'.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = b'.';
                } else {
                    break;
                }
            }
            for j in i + 2..7 {
                let cost = buf_traversal_cost(i + 2, j, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == b'.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = b'.';
                } else {
                    break;
                }
            }
        }
        res
    }

    fn transition_buffer_to_room(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..7 {
            if self.buffer[i] == b'.' {
                continue;
            }
            let r = (self.buffer[i] as u8 - b'A') as usize;
            if !self.is_valid_room(r) {
                continue;
            }
            if i <= r + 1 {
                if (i + 1..=r + 1).all(|i| self.buffer[i] == b'.') {
                    let mut next = self.clone();
                    let c = buf_traversal_cost(i, r + 1, next.buffer[i])
                        + weight(next.buffer[i])
                        + self.entry_cost(r);
                    next.rooms[r].push(next.buffer[i]);
                    next.buffer[i] = b'.';
                    res.push((next, c));
                }
            } else if (r + 2..i).all(|i| self.buffer[i] == b'.') {
                let mut next = self.clone();
                let c = buf_traversal_cost(r + 2, i, next.buffer[i])
                    + weight(next.buffer[i])
                    + self.entry_cost(r);
                next.rooms[r].push(next.buffer[i]);
                next.buffer[i] = b'.';
                res.push((next, c));
            }
        }
        res
    }

    fn transitions(&self) -> Vec<(State, i64)> {
        let mut res = self.transition_room_to_buffer();
        res.append(&mut self.transition_buffer_to_room());
        res
    }

    fn _print(&self) {
        println!("#############");
        print!("#");
        // TODO


    }
}

fn solve(input: State, expected: State) -> i64 {
    let mut costs = HashMap::new();
    let mut q = std::collections::BinaryHeap::new();
    costs.insert(input.clone(), 0);
    q.push((0, input));
    while let Some((cost, grid)) = q.pop() {
        let cost = -cost;
        if cost != costs[&grid] {
            continue;
        }
        if grid == expected {
            break;
        }
        for (transition, t_cost) in grid.transitions() {
            if let Some(&c) = costs.get(&transition) {
                if c <= t_cost + cost {
                    continue;
                }
            }
            costs.insert(transition.clone(), t_cost + cost);
            q.push((-(t_cost + cost), transition));
        }
    }
    *costs.get(&expected).unwrap()
}

pub fn day23() -> (String, String) {
    let part1 = solve(State::from_input(INPUT), State::final_state());

    (part1.to_string(), "".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let s = State::from_input(INPUT);
        assert_eq!(s.buffer, [b'.'; 7]);
        assert_eq!(s.rooms[0], [b'B', b'A']);
        assert_eq!(s.rooms[1], [b'C', b'D']);
        assert_eq!(s.rooms[2], [b'B', b'C']);
        assert_eq!(s.rooms[3], [b'D', b'A']);
    }
}
