const INPUT: &str = include_str!("../input/day11.txt");

struct Octopuses {
    octopuses: Vec<u8>,
    width: usize,
    height: usize,
    step_nb: usize,
}

impl Octopuses {
    pub fn new(input: &str) -> Octopuses {
        let width = input.find('\n').unwrap();
        let octopuses: Vec<u8> = input
            .as_bytes()
            .iter()
            .filter_map(|c| {
                if c.is_ascii_digit() {
                    Some(c - b'0')
                } else {
                    None
                }
            })
            .collect();
        let height = octopuses.len() / width;
        Octopuses {
            octopuses,
            width,
            height,
            step_nb: 0,
        }
    }

    fn pos_1d(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn pos_2d(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn flash(&mut self, i: usize, xy: (usize, usize)) {
        self.octopuses[i] += 2;
        let (x, y) = xy;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x.overflowing_add_signed(dx).0;
                let ny = y.overflowing_add_signed(dy).0;
                if nx < self.width && ny < self.height {
                    let n = self.pos_1d(nx, ny);
                    self.octopuses[n] += 1;
                    if self.octopuses[n] == 10 || self.octopuses[n] == 11 {
                        self.flash(n, (nx, ny));
                    }
                }
            }
        }
    }

    pub fn next_step(&mut self) -> u32 {
        self.step_nb += 1;
        let mut nb_flashes = 0;
        self.octopuses.iter_mut().for_each(|o| *o += 1);
        while let Some(i) = self.octopuses.iter().position(|&o| o == 10) {
            self.flash(i, self.pos_2d(i));
        }
        for o in self.octopuses.iter_mut() {
            if *o > 9 {
                *o = 0;
                nb_flashes += 1;
            }
        }
        // self._print();
        nb_flashes
    }

    pub fn part_1(&mut self) -> u32 {
        let mut nb_flashes = 0;
        for _ in 0..100 {
            nb_flashes += self.next_step();
        }
        nb_flashes
    }

    pub fn part_2(&mut self) -> u32 {
        while self.next_step() != (self.width * self.height) as u32 {}
        self.step_nb as u32
    }

    pub fn _print(&self) {
        println!("################");
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.octopuses[self.pos_1d(x, y)]);
            }
            println!();
        }
        println!("################");
    }
}

pub fn day11() -> (String, String) {
    let mut parsed = Octopuses::new(INPUT);
    let (part1, part2) = (parsed.part_1(), parsed.part_2());

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1_test_input_2_steps() {
        let mut parsed = Octopuses::new(TEST_INPUT);
        let mut nb_flashes = 0;
        nb_flashes += parsed.next_step();
        nb_flashes += parsed.next_step();
        assert_eq!(nb_flashes, 35);
    }

    #[test]
    fn test_part_1_test_input_100_steps() {
        let mut parsed = Octopuses::new(TEST_INPUT);
        assert_eq!(parsed.part_1(), 1656);
    }

    #[test]
    fn test_part_2_test_input() {
        let mut parsed = Octopuses::new(TEST_INPUT);
        assert_eq!(parsed.part_2(), 195);
    }

    #[test]
    fn test_part_1() {
        let mut parsed = Octopuses::new(INPUT);
        assert_eq!(parsed.part_1(), 1793);
    }

    #[test]
    fn test_part_2() {
        let mut parsed = Octopuses::new(INPUT);
        assert_eq!(parsed.part_2(), 247);
    }
}
