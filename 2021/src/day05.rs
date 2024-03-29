use std::iter;

const INPUT: &str = include_str!("../input/day05.txt");
const MAX_BOARD_SIZE: usize = 990;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Segment {
    a: Point,
    b: Point,
}

struct Board(Vec<u8>);

struct Parsed {
    board: Board,
    segments_straight: Vec<Segment>,
    segments_diagonal: Vec<Segment>,
}

impl Board {
    fn default() -> Self {
        Board(vec![0; MAX_BOARD_SIZE * MAX_BOARD_SIZE])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.0[y * MAX_BOARD_SIZE + x]
    }

    fn count_overlaps(&self) -> usize {
        self.0.iter().filter(|&&v| v > 1).count()
    }

    fn _print(&mut self) {
        println!("BOARD:");
        for y in 0..MAX_BOARD_SIZE {
            for x in 0..MAX_BOARD_SIZE {
                print!("{}", self.get_mut(x, y));
            }
            println!();
        }
    }
}

impl Point {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn parse(def: &str) -> Self {
        let mut split = def.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        Self::new(x, y)
    }
}

fn ez_range(a: isize, b: isize) -> impl Iterator<Item = isize> + Clone {
    let direction = (b - a).signum();
    iter::successors(Some(a), move |nb| Some(nb + direction))
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
        let range_x = ez_range(self.a.x, self.b.x).cycle();
        let range_y = ez_range(self.a.y, self.b.y).cycle();
        let length = (self.a.x - self.b.x).abs().max((self.a.y - self.b.y).abs()) as usize + 1;

        range_x
            .zip(range_y)
            .take(length)
            .map(|(x, y)| Point::new(x, y))
    }
}

impl Parsed {
    fn new(input: &str) -> Self {
        let (segments_diagonal, segments_straight) = input
            .split_terminator('\n')
            .map(Segment::parse)
            .partition(|s| s.is_diagonal());

        let board = Board::default();
        Self {
            segments_diagonal,
            segments_straight,
            board,
        }
    }
}

fn part1(input: &mut Parsed) -> String {
    for segment in input.segments_straight.iter() {
        for point in segment.iter() {
            *input.board.get_mut(point.x as usize, point.y as usize) += 1;
        }
    }
    input.board.count_overlaps().to_string()
}

fn part2(input: &mut Parsed) -> String {
    for segment in input.segments_diagonal.iter() {
        for point in segment.iter() {
            *input.board.get_mut(point.x as usize, point.y as usize) += 1;
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
