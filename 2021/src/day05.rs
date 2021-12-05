use heapless::Vec as StackVec;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input/day05.txt");
const MAX_SEGMENTS: usize = 500;
const BOARD_SIZE: usize = 990;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Segment {
    a: Point,
    b: Point,
}

/// ~1MB
struct Board([u8; BOARD_SIZE * BOARD_SIZE]);

struct Parsed {
    segments: StackVec<Segment, MAX_SEGMENTS>,
    board: Board,
}

impl Board {
    fn default() -> Self {
        Board([0; BOARD_SIZE * BOARD_SIZE])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.0[y * BOARD_SIZE + x]
    }

    fn count_overlaps(&self) -> usize {
        self.0.iter().filter(|&&v| v > 1).count()
    }

    fn _print(&mut self) {
        println!("BOARD:");
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                print!("{}", self.get_mut(x, y));
            }
            println!();
        }
    }
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn parse(def: &str) -> Self {
        let mut split = def.split(',');
        let x = split.next().unwrap().trim().parse::<usize>().unwrap();
        let y = split.next().unwrap().trim().parse::<usize>().unwrap();
        Self::new(x, y)
    }
}

fn ez_range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    match a.cmp(&b) {
        Ordering::Equal => box std::iter::repeat(a),
        Ordering::Less => box (a..=b),
        Ordering::Greater => box (b..=a).rev(),
    }
}

impl Segment {
    fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }

    fn parse(line: &str) -> Self {
        let mut pts = line.split(" -> ");
        let a = Point::parse(pts.next().unwrap());
        let b = Point::parse(pts.next().unwrap());
        Self::new(a, b)
    }

    fn is_diagonal(&self) -> bool {
        !(self.a.x == self.b.x || self.a.y == self.b.y)
    }

    fn iter(&self) -> impl Iterator<Item = Point> {
        let x_range = ez_range(self.a.x, self.b.x);
        let y_range = ez_range(self.a.y, self.b.y);
        let range = x_range.zip(y_range);
        range.map(|(x, y)| Point::new(x, y))
    }
}

impl Parsed {
    fn new(input: &str) -> Self {
        let segments = input
            .split_terminator('\n')
            .map(Segment::parse)
            .collect::<StackVec<_, MAX_SEGMENTS>>();
        let board = Board::default();
        Self { segments, board }
    }
}

/// No diagonals
fn part1(input: &mut Parsed) -> String {
    for segment in input.segments.iter().filter(|s| !s.is_diagonal()) {
        for point in segment.iter() {
            *input.board.get_mut(point.x, point.y) += 1;
        }
    }
    input.board.count_overlaps().to_string()
}

/// Build uppon part1 but add diagonals
fn part2(input: &mut Parsed) -> String {
    for segment in input.segments.iter().filter(|s| s.is_diagonal()) {
        for point in segment.iter() {
            *input.board.get_mut(point.x, point.y) += 1;
        }
    }
    input.board.count_overlaps().to_string()
}

pub fn day05() -> (String, String) {
    let mut parsed = Parsed::new(INPUT);

    (part1(&mut parsed), part2(&mut parsed))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn test_part1_test_input() {
        let mut parsed = Parsed::new(TEST_INPUT);
        assert_eq!(part1(&mut parsed), "5".to_string());
    }

    #[test]
    fn test_part2_test_input() {
        let mut parsed = Parsed::new(TEST_INPUT);
        part1(&mut parsed);
        assert_eq!(part2(&mut parsed), "12".to_string());
    }
}
