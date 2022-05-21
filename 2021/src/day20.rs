use bitvec::prelude::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day20.txt");

#[derive(Debug, Clone)]
struct Image {
    img: BitVec,
    size: usize,
    default_value: bool,
}

impl Image {
    fn from_bitvec(img: BitVec, size: usize) -> Self {
        Image {
            img,
            size,
            default_value: false,
        }
    }

    fn empty(size: usize) -> Self {
        let mut img = BitVec::with_capacity(size * size);
        img.resize(size * size, false);
        Image {
            img,
            size,
            default_value: false,
        }
    }

    fn pos_1d(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    pub fn get_px_index(&self, x: isize, y: isize) -> usize {
        let mut num: usize = 0;
        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (i, (dy, dx)) in deltas.iter().enumerate() {
            let x = x + dx;
            let y = y + dy;
            if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
                if self.default_value {
                    num |= 1 << (8 - i);
                }
            } else {
                num |= (self.img[self.pos_1d(x as usize, y as usize)] as usize) << (8 - i);
            }
        }
        num
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.img
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .chunks(self.size)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}

#[derive(Debug, Clone)]
struct Input {
    algorithm: BitVec,
    image: Image,
}

impl Input {
    pub fn from_str(s: &str) -> Self {
        let mut b_it = s.as_bytes().iter();
        let algorithm = b_it
            .by_ref()
            .take_while(|&&c| c != b'\n')
            .map(|&c| c == b'#')
            .collect::<BitVec>();
        let sep = *b_it.next().unwrap();
        debug_assert_eq!(sep, b'\n');
        let image_size = b_it.clone().take_while(|&&c| c != b'\n').count();
        let input_image = b_it
            .filter(|&&c| c != b'\n')
            .map(|&c| c == b'#')
            .collect::<BitVec>();
        debug_assert_eq!(input_image.len(), image_size * image_size);
        Self {
            algorithm,
            image: Image::from_bitvec(input_image, image_size),
        }
    }

    pub fn enhance_image(&mut self) {
        let mut new_img = Image::empty(self.image.size + 2);
        let isize = self.image.size as isize;
        for y in -1..isize + 1 {
            for x in -1..isize + 1 {
                let px_index = self.image.get_px_index(x, y);
                let idx = new_img.pos_1d((x + 1) as usize, (y + 1) as usize);
                new_img.img.set(idx, self.algorithm[px_index]);
            }
        }
        if *self.algorithm.get(0).as_deref().unwrap() {
            new_img.default_value = !self.image.default_value;
        }
        self.image = new_img;
    }
}

pub fn day20() -> (String, String) {
    let mut parsed = Input::from_str(INPUT);
    parsed.enhance_image();
    parsed.enhance_image();
    let p1 = parsed.image.img.count_ones().to_string();
    for _ in 0..(50 - 2) {
        parsed.enhance_image();
    }
    let p2 = parsed.image.img.count_ones().to_string();
    (p1, p2)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###";

    #[test]
    fn test_parsing() {
        println!("{}", TEST_INPUT);
        let parsed = Input::from_str(TEST_INPUT);
        let raw_image = TEST_INPUT.split("\n\n").nth(1).expect("WTF");
        assert_eq!(parsed.image.to_string(), raw_image);
    }

    #[test]
    fn test_image_enhance() {
        let mut parsed = Input::from_str(TEST_INPUT);
        parsed.enhance_image();
        assert_eq!(
            parsed.image.to_string(),
            ".##.##.\n\
            #..#.#.\n\
            ##.#..#\n\
            ####..#\n\
            .#..##.\n\
            ..##..#\n\
            ...#.#."
        );
        parsed.enhance_image();
        assert_eq!(
            parsed.image.to_string(),
            ".......#.\n\
            .#..#.#..\n\
            #.#...###\n\
            #...##.#.\n\
            #.....#.#\n\
            .#.#####.\n\
            ..#.#####\n\
            ...##.##.\n\
            ....###.."
        );
        for _ in 0..(50 - 2) {
            parsed.enhance_image();
        }
        assert_eq!(parsed.image.img.count_ones(), 3351);
    }
}
