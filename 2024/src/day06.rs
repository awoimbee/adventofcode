use std::hint::unreachable_unchecked;

use log::{debug, trace};

const INPUT: &str = include_str!("../input/day06.txt");

#[derive(Debug, Copy, Clone)]
/// 4 bits: cell type
/// 3 bits: visited direction p1
/// 3 bits: visited direction p2
/// ...
/// 16 bits: visitor_id (p2)
struct Cell(pub u32);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}

impl Direction {
    pub fn from_bits(bits: u32) -> Self {
        match bits {
            0b00 => Direction::None,
            1 => Direction::Left,
            2 => Direction::Right,
            3 => Direction::Up,
            4 => Direction::Down,
            _ => unreachable!(),
        }
    }

    pub fn from_gdir(offset: isize) -> Self {
        if offset < -1 {
            Self::Up
        } else if offset == -1 {
            Self::Left
        } else if offset == 1 {
            Self::Right
        } else if offset > 1 {
            Self::Down
        } else {
            unreachable!()
        }
    }
}

impl Cell {
    pub fn new(input: u8) -> Self {
        match input {
            b'.' => Self(0b0000),
            b'^' => Self(0b0010),
            b'#' => Self(0b0001),
            b'O' => Self(0b1001),
            b'\n' => Self(0b0110),
            _ => unreachable!(),
        }
    }

    pub fn is_blocked(&self) -> bool {
        (self.0 & 0b1) == 1
    }

    pub fn was_blocked(&self) -> bool {
        (self.0 & 0b1000) != 0
    }

    pub fn block(&mut self) {
        self.0 |= 0b1001;
    }

    pub fn unblock_keep_trace(&mut self) {
        self.0 &= !0b0111;
    }

    pub fn unblock_forget(&mut self) {
        self.0 &= !0b1111;
    }

    pub fn is_off_limit(&self) -> bool {
        (self.0 & 0b0111) == 0b0110
    }

    pub fn is_start(&self) -> bool {
        self.0 & 0b0111 == 0b0010
    }

    pub fn did_visit(&self, visitor_id: u32) -> Direction {
        let visited_by = self.0 >> 16;

        if visited_by == visitor_id {
            // part 2
            Direction::from_bits((self.0 >> 7) & 0b111)
        } else {
            // part 1
            Direction::from_bits((self.0 >> 4) & 0b111)
        }
    }

    pub fn set_visited_p1(&mut self, dir: Direction) {
        self.0 &= 0b1111111110001111;
        self.0 |= (dir as u32) << 4;
    }

    pub fn set_visited_p2(&mut self, visitor_id: u32, dir: Direction) {
        self.0 &= 0b1111110001111111;
        self.0 |= visitor_id << 16;
        self.0 |= (dir as u32) << 7;
    }
}

#[derive(Debug, Clone)]
struct Table {
    buf: Vec<Cell>,
    width: isize,
}

impl Table {
    pub fn new(input: &str) -> Self {
        let width = input.as_bytes().iter().position(|c| *c == b'\n').unwrap();
        let mut table = Vec::with_capacity(width * 2 + input.len());
        table.extend(std::iter::repeat(Cell::new(b'\n')).take(width));
        table.extend(input.as_bytes().iter().map(|b| Cell::new(*b)));
        table.extend(std::iter::repeat(Cell::new(b'\n')).take(width));

        Self {
            buf: table,
            width: width as isize + 1,
        }
    }

    pub fn as_str(&self, visitor_id: u32, guard_pos: usize) -> String {
        self.buf
            .iter()
            .enumerate()
            .map(|(id, cell)| {
                if cell.is_off_limit() {
                    '\n'
                } else if id == guard_pos {
                    'G'
                } else if cell.was_blocked() {
                    'O'
                } else if cell.is_blocked() {
                    '#'
                } else if cell.is_start() {
                    '^'
                } else if cell.did_visit(visitor_id) == Direction::Left {
                    '←'
                } else if cell.did_visit(visitor_id) == Direction::Up {
                    '↑'
                } else if cell.did_visit(visitor_id) == Direction::Down {
                    '↓'
                } else if cell.did_visit(visitor_id) == Direction::Right {
                    '→'
                } else {
                    '.'
                }
            })
            .collect::<String>()
            .trim()
            .to_owned()
    }
}

#[inline]
fn rotate(table: &Table, guard_dir: isize) -> isize {
    if guard_dir == -1 {
        -(table.width)
    } else if guard_dir == -(table.width) {
        1
    } else if guard_dir == 1 {
        table.width
    } else if guard_dir == (table.width) {
        -1
    } else {
        unsafe { unreachable_unchecked() };
    }
}

fn solve_is_a_loop(
    table: &mut Table,
    mut guard_pos: usize,
    mut guard_dir: isize,
    visitor_id: u32,
) -> bool {
    loop {
        if table.buf[guard_pos].is_off_limit() {
            return false;
        }
        trace!("p2\n{}", table.as_str(visitor_id, guard_pos));

        let mut next_pos = (guard_pos as isize + guard_dir) as usize;
        while table.buf[next_pos].is_blocked() {
            guard_dir = rotate(table, guard_dir);
            next_pos = (guard_pos as isize + guard_dir) as usize;
        }
        guard_pos = next_pos;
        let dir = Direction::from_gdir(guard_dir);
        if table.buf[guard_pos].did_visit(visitor_id) == dir {
            return true;
        }
        table.buf[guard_pos].set_visited_p2(visitor_id, dir);
    }
}

fn solve(input: &str) -> (u32, u32) {
    let mut table = Table::new(input);
    let mut guard_dir = -(table.width);
    let mut guard_pos = table.buf.iter().position(Cell::is_start).unwrap();

    let mut p2_visitor_id = 2;
    let mut p1 = 0;
    let mut p2 = 0;

    loop {
        trace!("p1\n{}", table.as_str(1, guard_pos));

        let gdir = Direction::from_gdir(guard_dir);
        if table.buf[guard_pos].did_visit(1) == Direction::None {
            table.buf[guard_pos].set_visited_p1(gdir);
            p1 += 1;
        }

        let mut next_pos = (guard_pos as isize + guard_dir) as usize;
        if table.buf[next_pos].is_blocked() {
            guard_dir = rotate(&table, guard_dir);
            next_pos = (guard_pos as isize + guard_dir) as usize;
        }
        if table.buf[next_pos].is_off_limit() {
            break;
        }

        if table.buf[next_pos].did_visit(1) == Direction::None {
            table.buf[next_pos].block();
            if solve_is_a_loop(&mut table, guard_pos, guard_dir, p2_visitor_id) {
                p2 += 1;
                debug!(
                    "obstacle @ {} {}",
                    guard_pos / (table.width as usize) - 1,
                    guard_pos % (table.width as usize)
                );
                table.buf[next_pos].unblock_keep_trace();
            } else {
                table.buf[next_pos].unblock_forget();
            }
            p2_visitor_id += 1;
        }
        guard_pos = next_pos;
    }
    debug!("Final State:\n{}", table.as_str(1, guard_pos));
    (p1, p2)
}

pub fn day06() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part_1() {
        init();
        assert_eq!(solve(TEST_INPUT).0, 41);
    }

    #[test]
    fn test_part_2() {
        init();
        assert_eq!(solve(TEST_INPUT).1, 6);
    }

    #[test]
    fn test_part_2_2() {
        init();
        assert_eq!(
            solve(
                "...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......"
            )
            .1,
            19
        );
    }
}
