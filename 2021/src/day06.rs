const INPUT: &str = include_str!("../input/day06.txt");
const NB_DAYS: usize = 9;

struct Fishes {
    fishes: [u64; NB_DAYS],
    offset: usize,
}

impl Fishes {
    fn new(input: &str) -> Self {
        let mut fishes = [0; NB_DAYS];
        input
            .trim_end()
            .split(',')
            .for_each(|fish| fishes[fish.parse::<usize>().unwrap()] += 1);
        Self { fishes, offset: 0 }
    }

    fn index(&self, i: usize) -> usize {
        (self.offset + i) % NB_DAYS
    }

    fn next_day(&mut self) {
        self.offset = self.index(1);
        self.fishes[self.index(6)] += self.fishes[self.index(8)];
    }

    fn sum(&self) -> u64 {
        self.fishes.iter().sum()
    }

    fn _print(&self) {
        for i in 0..NB_DAYS {
            print!("{} ", self.fishes[self.index(i)]);
        }
        println!();
    }
}

pub fn day06() -> (String, String) {
    let mut fishes = Fishes::new(INPUT);

    for _ in 0..80 {
        fishes.next_day();
    }
    let part1 = fishes.sum();
    for _ in 0..(256 - 80) {
        fishes.next_day();
    }
    let part2 = fishes.sum();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"3,4,3,1,2"#;

    #[test]
    fn test_parsing_test_input() {
        let fishes = Fishes::new(TEST_INPUT);
        assert_eq!(fishes.fishes, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_test_input_18_days() {
        let mut parsed = Fishes::new(TEST_INPUT);
        for _ in 0..18 {
            parsed.next_day();
        }
        assert_eq!(parsed.sum(), 26);
    }

    #[test]
    fn test_test_input_80_days() {
        let mut parsed = Fishes::new(TEST_INPUT);
        for _ in 0..80 {
            parsed.next_day();
        }
        assert_eq!(parsed.sum(), 5934);
    }

    #[test]
    fn test_80_days() {
        let mut parsed = Fishes::new(INPUT);
        for _ in 0..80 {
            parsed.next_day();
        }
        assert_eq!(parsed.sum(), 345387);
    }

    #[test]
    fn test_256_days() {
        let mut parsed = Fishes::new(INPUT);
        for _ in 0..256 {
            parsed.next_day();
        }
        assert_eq!(parsed.sum(), 1574445493136);
    }
}
