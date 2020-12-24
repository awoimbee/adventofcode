const INPUT: &str = include_str!("../input/day05.txt");
use bitvec::prelude::*;

#[derive(Debug)]
struct Rule {
    pub char: u8,
    pub min: u8,
    pub max: u8,
}

struct Range {
    min: u32,
    max: u32,
}
impl Range {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
    pub fn divide(&mut self, upper_half: bool) {
        // println!("defore div {} {}", self.min, self.max);
        let middle = (self.max - self.min) / 2 + self.min;
        if upper_half {
            self.min = middle + 1;
        } else {
            self.max = middle;
        }
        // println!("div {} {}", self.min, self.max);
    }
    pub fn final_value(&self) -> u32 {
        // println!("{} {}", self.min, self.max);
        assert!(self.min == self.max);
        self.min
    }
}

struct Seat {
    pub row: u8, // 128 row
    pub col: u8, // 8 col
}

impl Seat {
    pub fn new(id: &str) -> Self {
        let mut row = Range::new(0, 127);
        let mut col = Range::new(0, 7);
        // println!("{}", id);
        for c in id.as_bytes() {
            match c {
                b'B' => row.divide(true),
                b'F' => row.divide(false),
                b'L' => col.divide(false),
                b'R' => col.divide(true),
                _ => unreachable!(),
            }
        }
        Self {
            row: row.final_value() as u8,
            col: col.final_value() as u8,
        }
    }
    pub fn id(&self) -> u32 {
        self.row as u32 * 8 + self.col as u32
    }
}

fn parse() -> Vec<Seat> {
    INPUT.lines().map(Seat::new).collect()
}

pub fn day05() -> (String, String) {
    let input = parse();

    let mut seats_taken = bitvec![0; 1024];

    let mut p1 = 0;
    for s in input {
        let s_id = s.id();
        if s_id > p1 {
            p1 = s_id;
        }
        seats_taken.set(s_id as usize, true);
    }

    let mut p2 = 0;
    for (idx, s_taken) in seats_taken.iter().enumerate().skip(1) {
        if !s_taken && seats_taken[idx - 1] && seats_taken[idx + 1] {
            p2 = idx;
        }
    }

    (p1.to_string(), p2.to_string())
}
