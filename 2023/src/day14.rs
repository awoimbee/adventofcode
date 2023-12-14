use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day14.txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Platform {
    width: usize,
    height: usize,
    rocks: Vec<u8>,
}

impl Platform {
    fn tilt_north(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                if self.rocks[i] == b'O' {
                    let mut v = y;
                    while v != 0 && self.rocks[x + (v - 1) * self.width] == b'.' {
                        v -= 1;
                    }
                    self.rocks[i] = b'.';
                    self.rocks[x + v * self.width] = b'O';
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.width {
            for y in (0..self.height).rev() {
                let i = x + y * self.width;
                if self.rocks[i] == b'O' {
                    let mut v = y;
                    while v != self.height - 1 && self.rocks[x + (v + 1) * self.width] == b'.' {
                        v += 1;
                    }
                    self.rocks[i] = b'.';
                    self.rocks[x + v * self.width] = b'O';
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = x + y * self.width;
                if self.rocks[i] == b'O' {
                    let mut v = x;
                    while v != 0 && self.rocks[(v - 1) + y * self.width] == b'.' {
                        v -= 1;
                    }
                    self.rocks[i] = b'.';
                    self.rocks[v + y * self.width] = b'O';
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                let i = x + y * self.width;
                if self.rocks[i] == b'O' {
                    let mut v = x;
                    while v != self.width - 1 && self.rocks[(v + 1) + y * self.width] == b'.' {
                        v += 1;
                    }
                    self.rocks[i] = b'.';
                    self.rocks[v + y * self.width] = b'O';
                }
            }
        }
    }

    fn calculate_load(&self) -> u64 {
        let mut sum = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                if self.rocks[i] == b'O' {
                    sum += (self.height - y) as u64;
                }
            }
        }
        sum
    }
}

fn solve(input: &str) -> (u64, u64) {
    let platform = input
        .as_bytes()
        .iter()
        .filter(|&&c| c != b'\n')
        .copied()
        .collect::<Vec<_>>();
    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mut platform = Platform {
        width,
        height,
        rocks: platform,
    };
    let mut cache: HashMap<Platform, usize> = HashMap::new();
    cache.insert(platform.clone(), 0);

    platform.tilt_north();
    let p1 = platform.calculate_load();

    for i in 0..1000000000 {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();

        if let Some(cached_i) = cache.get(&platform) {
            // we went back to a previous state => we are in a repeating cycle
            let cycle = i + 1 - cached_i;
            let remaining = (1_000_000_000 - i - 1) % cycle;
            for _ in 0..remaining {
                platform.tilt_north();
                platform.tilt_west();
                platform.tilt_south();
                platform.tilt_east();
            }
            break;
        }
        cache.insert(platform.clone(), i + 1);
    }
    let p2 = platform.calculate_load();

    (p1, p2)
}

pub fn day14() -> (String, String) {
    let (p1, p2) = solve(INPUT);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "#};

    #[test]
    fn test_p1() {
        assert_eq!(solve(TEST_INPUT).0, 136);
    }


    #[test]
    fn test_p2() {
        assert_eq!(solve(TEST_INPUT).1, 64);
    }


}
