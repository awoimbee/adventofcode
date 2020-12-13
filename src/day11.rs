use rayon::prelude::*;
use std::cell::UnsafeCell;

const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day11.txt")) };

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

pub struct UnsafeVec<'a>(UnsafeCell<&'a mut Vec<&'a mut [u8]>>);
unsafe impl<'a> Sync for UnsafeVec<'a> {}

pub struct UnsafeBool(UnsafeCell<bool>);
unsafe impl Sync for UnsafeBool {}

type NeighborCounter = fn(&[&mut [u8]], usize, usize) -> usize;

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
}

fn parse() -> Map {
    INPUT
        .split('\n')
        .filter(|s| !str::is_empty(s))
        .map(|s| s.as_bytes())
        .fold(Map::default(), |acc, x| acc.accumulate_row(x))
}

fn count_neighbors1(seats: &[&mut [u8]], y: usize, x: usize) -> usize {
    let width = seats[0].len();
    let height = seats.len();
    let mut nb = 0;

    for (of_y, of_x) in DIRECTIONS.iter() {
        let tmp_y = y as isize + of_y;
        let tmp_x = x as isize + of_x;
        if !(0..width as isize).contains(&tmp_x) || !(0..height as isize).contains(&tmp_y) {
            continue;
        }
        if seats[tmp_y as usize][tmp_x as usize] == OCCUPIED {
            nb += 1;
        }
    }
    nb
}

fn search(x: isize, y: isize, dx: isize, dy: isize, seats: &[&mut [u8]]) -> usize {
    if x < 0 || x >= seats[0].len() as isize || y < 0 || y >= seats.len() as isize {
        return 0;
    }
    match seats[y as usize][x as usize] {
        FLOOR => search(x + dx, y + dy, dx, dy, seats),
        OCCUPIED => 1,
        EMPTY => 0,
        _ => unreachable!(),
    }
}

fn count_neighbors2(seats: &[&mut [u8]], y: usize, x: usize) -> usize {
    DIRECTIONS
        .iter()
        .map(|&(dx, dy)| search(x as isize + dx, y as isize + dy, dx, dy, seats))
        .sum()
}

fn run_simulation(
    seats0: &mut Vec<u8>,
    width: usize,
    height: usize,
    tolerance: usize,
    count_neighbors: NeighborCounter,
) {
    let mut seats1 = seats0.clone();
    let mut seats_view0 = seats0.chunks_exact_mut(width).collect::<Vec<_>>();
    let mut seats_view1 = seats1.chunks_exact_mut(width).collect::<Vec<_>>();
    let mut sv_ref0 = UnsafeVec(UnsafeCell::from(&mut seats_view0));
    let mut sv_ref1 = UnsafeVec(UnsafeCell::from(&mut seats_view1));
    let updated = UnsafeBool(UnsafeCell::from(true));
    let range: Vec<usize> = (0..(height * width)).collect();
    unsafe {
        while *updated.0.get() {
            *updated.0.get() = false;
            range.par_iter().for_each(|i| {
                let x = i % width;
                let y = i / width;
                let seat = (*sv_ref0.0.get())[y][x];
                if seat == FLOOR {
                    return;
                };
                let nb_neighbors = count_neighbors(*sv_ref0.0.get(), y, x);

                (*sv_ref1.0.get())[y][x] = if seat == OCCUPIED && nb_neighbors >= tolerance {
                    *updated.0.get() = true;
                    EMPTY
                } else if seat != OCCUPIED && nb_neighbors == 0 {
                    *updated.0.get() = true;
                    OCCUPIED
                } else {
                    seat
                };
            });
            let t = sv_ref0;
            sv_ref0 = sv_ref1;
            sv_ref1 = t;
        }
    }
}

pub fn day11() -> (String, String) {
    let mut map = parse();
    let mut map2_seats = map.seats.clone();

    run_simulation(&mut map.seats, map.width, map.height, 4, count_neighbors1);
    let p1: u32 = map
        .seats
        .iter()
        .map(|s| if *s == b'#' { 1 } else { 0 })
        .sum();
    run_simulation(&mut map2_seats, map.width, map.height, 5, count_neighbors2);
    let p2: u32 = map2_seats
        .iter()
        .map(|s| if *s == b'#' { 1 } else { 0 })
        .sum();

    (format!("{}", p1), format!("{}", p2))
}
