const INPUT: &str = include_str!("../input/day11.txt");

const FLOOR: u8 = b'.';
const OCCUPIED: u8 = b'#';
const EMPTY: u8 = b'L';

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
struct Map {
    pub seats: Vec<u8>,
    pub width: usize,
    pub height: usize,
}
impl Map {
    pub fn default() -> Self {
        Self {
            seats: Vec::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn accumulate_row(mut self, row: &[u8]) -> Self {
        self.height += 1;
        self.width = row.len();
        self.seats.extend_from_slice(row);
        self
    }
    pub fn seats_indices_part1(&self) -> Vec<(usize, Vec<usize>)> {
        self.seats
            .iter()
            .enumerate()
            .filter(|(_, s)| **s != FLOOR)
            .map(|(i, _)| {
                let x = (i % self.width) as isize;
                let y = (i / self.width) as isize;
                (
                    i,
                    DIRECTIONS
                        .iter()
                        .filter_map(|(of_y, of_x)| {
                            let tmp_y = y + of_y;
                            let tmp_x = x + of_x;
                            if !(0..self.width as isize).contains(&tmp_x)
                                || !(0..self.height as isize).contains(&tmp_y)
                            {
                                None
                            } else {
                                let idx = tmp_y as usize * self.width + tmp_x as usize;
                                if self.seats[idx] == FLOOR {
                                    None
                                } else {
                                    Some(idx)
                                }
                            }
                        })
                        .collect::<Vec<usize>>(),
                )
            })
            .collect()
    }
    pub fn seats_indices_part2(&self) -> Vec<(usize, Vec<usize>)> {
        self.seats
            .iter()
            .enumerate()
            .filter(|(_, s)| **s != FLOOR)
            .map(|(i, _)| {
                (
                    i,
                    (0..9)
                        .filter(|r| r != &4)
                        .map(|r| (r % 3 - 1, r / 3 - 1))
                        .filter_map(|(rx, ry)| {
                            (1..)
                                .map(|f| {
                                    (
                                        (i as isize % self.width as isize) + rx * f,
                                        (i as isize / self.width as isize) + ry * f,
                                    )
                                })
                                .take_while(|(x, y)| {
                                    *x >= 0
                                        && *y >= 0
                                        && *x < self.width as isize
                                        && *y < self.height as isize
                                })
                                .map(|(x, y)| (y * self.width as isize + x) as usize)
                                .find(|i| self.seats[*i] == EMPTY)
                        })
                        .collect(),
                )
            })
            .collect()
    }
}

fn parse() -> Map {
    INPUT
        .lines()
        .map(|s| s.as_bytes())
        .fold(Map::default(), |acc, x| acc.accumulate_row(x))
}

fn run_simulation(
    mut seats: Vec<u8>,
    seat_map: Vec<(usize, Vec<usize>)>,
    tolerance: usize,
) -> usize {
    let mut seats1 = seats.clone();

    let mut cur = &mut seats;
    let mut next = &mut seats1;

    unsafe {
        let mut updated = true;
        while updated {
            updated = false;
            for (s_id, s_neigh) in seat_map.iter() {
                let nb_neighbors: usize = s_neigh
                    .iter()
                    .map(|&s_id| (*cur.get_unchecked(s_id) == OCCUPIED) as usize)
                    .sum();
                let seat = cur[*s_id];
                *next.get_unchecked_mut(*s_id) = if seat == OCCUPIED && nb_neighbors >= tolerance {
                    updated = true;
                    EMPTY
                } else if seat != OCCUPIED && nb_neighbors == 0 {
                    updated = true;
                    OCCUPIED
                } else {
                    seat
                }
            }
            std::mem::swap(&mut cur, &mut next);
        }
    }
    seats.iter().map(|s| if *s == b'#' { 1 } else { 0 }).sum()
}

pub fn day11() -> (String, String) {
    let map = parse();
    let map2 = map.clone();

    let p1_map = map.seats_indices_part1();
    let p1 = run_simulation(map.seats, p1_map, 4);

    let p2_map = map2.seats_indices_part2();
    let p2 = run_simulation(map2.seats, p2_map, 5);

    (p1.to_string(), p2.to_string())
}
