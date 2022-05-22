use fnv::FnvHashMap;
use heapless::Vec as StackVec;
use itertools::Itertools;

const INPUT_P1: &str = include_str!("../input/day23_1.txt");
const INPUT_P2: &str = include_str!("../input/day23_2.txt");

const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const BUF_WEIGHT: [i64; 7] = [0, 1, 3, 5, 7, 9, 10];

fn weight(c: u8) -> i64 {
    WEIGHT[(c - b'A') as usize]
}

fn buf_traversal_cost(i: usize, j: usize, c: u8) -> i64 {
    (BUF_WEIGHT[i] - BUF_WEIGHT[j]).abs() * WEIGHT[(c - b'A') as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State<const ROOM_SIZE: usize> {
    rooms: [StackVec<u8, ROOM_SIZE>; 4],
    buffer: [u8; 7],
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
    fn from_input(input: &str) -> Self {
        debug_assert!(input.starts_with("#############\n#"));
        let raw_buffer = &input.as_bytes()[15..26];
        let mut raw_rows = input[28..].lines().rev();
        let term_row = raw_rows.next().unwrap();
        debug_assert_eq!(term_row, "  #########  ");

        let mut buffer = [b'.'; 7];
        {
            let mut j = 0;
            for (i, &val) in raw_buffer.iter().enumerate() {
                if [3, 5, 7, 9].contains(&i) {
                    continue;
                };
                buffer[j] = val;
                j += 1;
            }
        }
        let mut rooms = [
            StackVec::new(),
            StackVec::new(),
            StackVec::new(),
            StackVec::new(),
        ];
        for r in raw_rows {
            let rb = r.as_bytes();
            rooms[0].push(rb[3]).unwrap();
            rooms[1].push(rb[5]).unwrap();
            rooms[2].push(rb[7]).unwrap();
            rooms[3].push(rb[9]).unwrap();
        }

        for r in rooms.iter() {
            debug_assert_eq!(r.len(), ROOM_SIZE);
        }
        debug_assert!(!rooms.iter().flatten().contains(&b'#'));
        debug_assert!(!buffer.iter().contains(&b'#'));

        Self { rooms, buffer }
    }

    fn final_state() -> Self {
        match ROOM_SIZE {
            2 => State {
                rooms: [
                    StackVec::from_slice(&[b'A', b'A']).unwrap(),
                    StackVec::from_slice(&[b'B', b'B']).unwrap(),
                    StackVec::from_slice(&[b'C', b'C']).unwrap(),
                    StackVec::from_slice(&[b'D', b'D']).unwrap(),
                ],
                buffer: [b'.'; 7],
            },
            4 => State {
                rooms: [
                    StackVec::from_slice(&[b'A', b'A', b'A', b'A']).unwrap(),
                    StackVec::from_slice(&[b'B', b'B', b'B', b'B']).unwrap(),
                    StackVec::from_slice(&[b'C', b'C', b'C', b'C']).unwrap(),
                    StackVec::from_slice(&[b'D', b'D', b'D', b'D']).unwrap(),
                ],
                buffer: [b'.'; 7],
            },
            _ => panic!("Room size not handled: {}", ROOM_SIZE),
        }
    }

    fn is_valid_room(&self, i: usize) -> bool {
        self.rooms[i].iter().all(|&c| i == (c - b'A') as usize)
    }

    fn entry_cost(&self, i: usize) -> i64 {
        (ROOM_SIZE - self.rooms[i].len()) as i64 * WEIGHT[i]
    }

    fn exit_cost(&self, i: usize, c: u8) -> i64 {
        (ROOM_SIZE - self.rooms[i].len()) as i64 * weight(c)
    }

    fn transition_room_to_buffer(&self) -> Vec<(State<ROOM_SIZE>, i64)> {
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

    fn transition_buffer_to_room(&self) -> Vec<(State<ROOM_SIZE>, i64)> {
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
                    next.rooms[r].push(next.buffer[i]).unwrap();
                    next.buffer[i] = b'.';
                    res.push((next, c));
                }
            } else if (r + 2..i).all(|i| self.buffer[i] == b'.') {
                let mut next = self.clone();
                let c = buf_traversal_cost(r + 2, i, next.buffer[i])
                    + weight(next.buffer[i])
                    + self.entry_cost(r);
                next.rooms[r].push(next.buffer[i]).unwrap();
                next.buffer[i] = b'.';
                res.push((next, c));
            }
        }
        res
    }

    fn transitions(&self) -> Vec<(State<ROOM_SIZE>, i64)> {
        let mut res = self.transition_room_to_buffer();
        res.append(&mut self.transition_buffer_to_room());
        res
    }
}

impl<const ROOM_SIZE: usize> std::fmt::Display for State<ROOM_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "#############")?;
        let b = self.buffer.iter().map(|&b| b as char).collect::<Vec<_>>();
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6]
        )?;
        let mut rooms = self
            .rooms
            .iter()
            .map(|r| r.iter().map(|&b| b as char))
            .collect::<Vec<_>>();

        let mut lines = (0..ROOM_SIZE)
            .map(|_| {
                format!(
                    "  #{}#{}#{}#{}#  ",
                    rooms[0].next().unwrap_or('.'),
                    rooms[1].next().unwrap_or('.'),
                    rooms[2].next().unwrap_or('.'),
                    rooms[3].next().unwrap_or('.')
                )
            })
            .collect::<Vec<_>>();
        unsafe {
            let lol = lines.last_mut().unwrap().as_bytes_mut();
            lol[0] = b'#';
            lol[1] = b'#';
            lol[11] = b'#';
            lol[12] = b'#';
        }
        for l in lines.into_iter().rev() {
            writeln!(f, "{}", l)?;
        }
        write!(f, "  #########  ")?;

        Ok(())
    }
}

fn solve<const ROOM_SIZE: usize>(input: State<ROOM_SIZE>, expected: State<ROOM_SIZE>) -> i64 {
    let mut costs = FnvHashMap::default();
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
    let part1 = solve(State::<2>::from_input(INPUT_P1), State::final_state());
    let part2 = solve(State::<4>::from_input(INPUT_P2), State::final_state());

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT_P1: &str =
        "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#  \n  #########  ";
    const TEST_INPUT_P2: &str =
        "#############\n#...........#\n###B#C#B#D###\n  #D#C#B#A#  \n  #D#B#A#C#  \n  #A#D#C#A#  \n  #########  ";

    #[test]
    fn test_parsing_p1() {
        let s = State::<2>::from_input(TEST_INPUT_P1);
        assert_eq!(s.to_string(), TEST_INPUT_P1);
    }

    #[test]
    fn test_parsing_p2() {
        let s = State::<4>::from_input(TEST_INPUT_P2);
        assert_eq!(s.to_string(), TEST_INPUT_P2);
    }

    #[test]
    fn test_part_1() {
        let input = State::<2>::from_input(TEST_INPUT_P1);
        assert_eq!(solve(input, State::final_state()), 12521);
    }

    #[test]
    fn test_part_2() {
        let input = State::<4>::from_input(TEST_INPUT_P2);
        assert_eq!(solve(input, State::final_state()), 44169);
    }
}
