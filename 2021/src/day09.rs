const INPUT: &str = include_str!("../input/day09.txt");

struct Map {
    data: Vec<u8>,
    offsets: [isize; 4],
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let data = input
            .split_terminator('\n')
            .flat_map(|line| line.as_bytes().iter().map(|b| b - b'0'))
            .collect();
        let w = width as isize;
        let offsets = [-1, 1, -w, w];
        Self {
            data,
            width,
            offsets,
        }
    }

    fn _print(&self) {
        for chunk in self.data.chunks(self.width) {
            for &b in chunk {
                print!(
                    "{}",
                    if b == 9 {
                        '#'
                    } else if b == 10 {
                        '+'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    #[inline]
    fn iter_neighbors(&'_ self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        self.offsets
            .into_iter()
            .filter(move |offset| {
                let result = idx.overflowing_add_signed(*offset).0;
                result < self.data.len()
                    && ((idx % self.width) == (result % self.width)
                        || (idx / self.width) == (result / self.width))
            })
            .map(move |offset| idx.overflowing_add_signed(offset).0)
    }

    fn local_low_points(&self) -> impl Iterator<Item = (usize, &u8)> {
        self.data.iter().enumerate().filter(move |(i, point)| {
            self.iter_neighbors(*i)
                .all(|neighbor| **point < self.data[neighbor])
        })
    }

    fn recurse_bassin(&mut self, pos: usize) -> u64 {
        let mut sum = 1;
        self.data[pos as usize] = 10;
        for neighbor in self.iter_neighbors(pos).collect::<Vec<_>>().into_iter() {
            match self.data.get(neighbor) {
                Some(9) | Some(10) | None => continue,
                Some(_) => sum += self.recurse_bassin(neighbor),
            }
        }
        sum
    }
}

fn solve(mut input: Map) -> (u32, u64) {
    let mut part1 = 0;
    let mut low_points_indexes = Vec::new();
    for (idx, low_point) in input.local_low_points() {
        part1 += (*low_point + 1) as u32;
        low_points_indexes.push(idx);
    }
    let mut bassins = low_points_indexes
        .into_iter()
        .map(|i| input.recurse_bassin(i))
        .collect::<Vec<_>>();

    bassins.sort_unstable();

    let part2 = bassins.into_iter().rev().take(3).product();
    (part1, part2)
}

pub fn day09() -> (String, String) {
    let parsed = Map::new(INPUT);
    let (part1, part2) = solve(parsed);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1_test_input() {
        let map = Map::new(TEST_INPUT);
        assert_eq!(solve(map).0, 15);
    }

    #[test]
    fn test_part_2_test_input() {
        let map = Map::new(TEST_INPUT);
        assert_eq!(solve(map).1, 1134);
    }
}
